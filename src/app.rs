use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use audio_devices::{get_devices, PaSourceInfo};
use audio_process::{AudioProcessor, FRAMES};
use config::{ConvertTo, GtkVisualizerConfig};
use instance::GtkVisualizerInstance;
use gtk;
use gtk::{StatusIcon, Window, WindowType};

pub enum UpdateMessage {
    // id, index
    Destroy(usize, usize),
    // id, old index, new index
    ChangeMapping(usize, usize, usize),
}

pub type AudioFrame = Vec<Vec<f64>>;

// NOTE: include the icon as bytes in the program
pub struct GtkVisualizerApp {
    // id needed for configs and title
    // when assigning: give current, then increment
    current_id_n: usize,
    pub instances: HashMap<usize, GtkVisualizerInstance>,
    default_source_name: String,
    pub sources: Vec<Option<PaSourceInfo>>,
    // multiple renderers per audio processor - have list of processors and map them to avoid
    // overuse of audio resources and conflicts
    // array w/ size of max index + 1
    // vec<usize> are the ids
    pub audio_processor_mappings: Vec<Option<(AudioProcessor, Vec<usize>)>>,
    icon: StatusIcon,
    // receiver for deletion messages
    msg_receiver: Receiver<UpdateMessage>,
    pub current_data: Vec<Option<Arc<Mutex<AudioFrame>>>>,
}

impl GtkVisualizerApp {
    // Only call this once
    fn initialize() -> GtkVisualizerApp {
        gtk::init().expect("Failed to initialize GTK");
        // make a test window to test whether composited
        let test_window = Window::new(WindowType::Toplevel);
        let screen = WindowExt::get_screen(&test_window).unwrap();
        assert!(screen.is_composited(), "Gtk is not composited");
        drop(screen);
        drop(test_window);

        let (default_source, devices) = get_devices().expect("Could not get any audio devices");
        let mut instances = HashMap::<usize, GtkVisualizerInstance>::new();
        let mut audio_processor_mappings = Vec::new();
        let (update_send, update_recv) = channel::<UpdateMessage>();
        // read from configs and initialize instances here
        // ...
        // ...
        let mut current_data = Vec::new();
        // for (id, mut instance) in instances.iter_mut() {
        // let data_source = current_data.entry(instance.index())
        // .or_insert(Arc::new(Mutex::new(vec![vec![0f64; FRAMES]])));
        // instance.set_sender(update_send.clone());
        // instance.set_data_source(data_source.clone());
        // instance.show_all();
        // }
        // initialize and set icon callbacks
        // ...
        // ...
        // use a channel for messages too? or just use update_send and add more message variants
        GtkVisualizerApp {
            current_id_n: instances.len(),
            instances: instances,
            default_source_name: default_source,
            sources: devices,
            audio_processor_mappings: audio_processor_mappings,
            icon: StatusIcon::new_from_file("icon.png"),
            msg_receiver: update_recv,
            current_data: current_data,
        };
        unimplemented!()
    }

    // Update source information - this should never happen while the program is running afaik - it
    // would require restarting all instances with new data
    // fn update_sources(&mut self) -> Result<(), String> {
    // let source_info = try!(get_devices());
    // self.default_source_name = source_info.0;
    // self.sources = source_info.1;
    // Ok(())
    // }

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
                self.instances.remove(&id);
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

    pub fn main_iteration(&mut self) -> Result<(), String> {
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

        // iterate instances
        for instance in self.instances.iter_mut().map(|(_, i)| i) {
            instance.iterate();
        }

        // run the actual gtk iteration
        gtk::main_iteration();
        Ok(())
    }
}

impl ConvertTo<Vec<GtkVisualizerConfig>> for GtkVisualizerApp {
    fn convert_to(&self) -> Vec<GtkVisualizerConfig> {
        self.instances.values().map(|v| v.convert_to()).collect()
    }
}
