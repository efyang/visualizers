use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use audio_devices::{get_devices, PaSourceInfo};
use audio_process::{AudioProcessor, FRAMES, AudioFrame};
use message::UpdateMessage;

pub struct AudioUpdater {
    default_source_name: String,
    pub sources: Vec<Option<PaSourceInfo>>,
    // multiple renderers per audio processor - have list of processors and map them to avoid
    // overuse of audio resources and conflicts
    // array w/ size of max index + 1
    // vec<usize> are the ids
    pub audio_processor_mappings: Vec<Option<(AudioProcessor, Vec<usize>)>>,
    // receiver for deletion messages
    msg_receiver: Receiver<UpdateMessage>,
    pub current_data: Vec<Option<Arc<Mutex<AudioFrame>>>>,
    program_continue: Arc<Mutex<bool>>,
}

impl AudioUpdater {
    pub fn new(default_source_name: &str, sources: Vec<Option<PaSourceInfo>>, audio_processor_mappings: Vec<Option<(AudioProcessor, Vec<usize>)>>, msg_receiver: Receiver<UpdateMessage>, data: Vec<Option<Arc<Mutex<AudioFrame>>>>, program_continue: Arc<Mutex<bool>>) -> Self {
        AudioUpdater {
            default_source_name: default_source_name.to_string(),
            sources: sources,
            audio_processor_mappings: audio_processor_mappings,
            msg_receiver: msg_receiver,
            current_data: data,
            program_continue: program_continue,
        }
    }

    fn default_source_index(&mut self) -> Option<usize> {
        for i in 0..self.sources.len() {
            if let Some(ref source_info) = self.sources[i] {
                if source_info.name == self.default_source_name {
                    return Some(i);
                }
            }
        }
        None
    }

    pub fn remove_id_from_index(&mut self, id: usize, index: usize) {
        let mut rm_audio_processor = false;
        if let Some((_, ref mut ids)) = self.audio_processor_mappings[index] {
            for i in 0..ids.len() {
                if ids[i] == id {
                    ids.swap_remove(i);
                }
            }
            if ids.len() == 0 {
                rm_audio_processor = true;
            }
        }
        // remove audio processor if no more instances are using it
        if rm_audio_processor {
            self.audio_processor_mappings[index] = None;
            self.current_data[index] = None;
        }
    }

    pub fn assign_id_to_index(&mut self, id: usize, index: usize) -> Result<(), String> {
        if let Some((_, ref mut ids)) = self.audio_processor_mappings[index] {
            ids.push(id);
            return Ok(());
        }
        // if the processor doesn't exist, create it
        match AudioProcessor::new(self.sources.as_slice(), index) {
            Some(processor) => {
                self.current_data[index] =
                    Some(Arc::new(Mutex::new(vec![vec![0f64; FRAMES]; processor.channels()])));
                self.audio_processor_mappings[index] = Some((processor, vec![id]));
                Ok(())
            }
            None => Err(format!("Could not set id {} to index {}", id, index)),
        }
    }

    fn handle_message(&mut self, message: UpdateMessage) -> Result<(), String> {
        match message {
            UpdateMessage::Destroy(id, index) => {
                self.remove_id_from_index(id, index);
            }
            UpdateMessage::ChangeMapping(id, old_idx, new_idx) => {
                try!(self.assign_id_to_index(id, new_idx));
                self.remove_id_from_index(id, old_idx);
                // set it to the new data source
            }
        }
        Ok(())
    }

    pub fn iterate(&mut self) -> Result<(), String> {
        // check all messages
        match self.msg_receiver.try_recv() {
            Ok(m) => try!(self.handle_message(m)),
            Err(e) => return Err(format!("{}", e)),
        }

        // set data from audio processors
        for mapping in self.audio_processor_mappings.iter_mut() {
            if let Some((ref mut processor, _)) = *mapping {
                let data = processor.get_data_frame();
                *self.current_data[processor.source_index()].as_ref().unwrap().lock().unwrap() =
                    data;
            }
        }
        Ok(())
    }
}
