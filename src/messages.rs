use std::sync::mpsc::{Sender, Receiver};
use audio_process::AudioProcessor;
use visualize::{GtkVisualizerApp, GtkVisualizerInstance, DataMessage, UpdateMessage};

pub trait HandleMessage {
    type M;
    fn handle_message(&mut self, message: Self::M) -> Result<(), String>;
}

impl HandleMessage for GtkVisualizerApp {
    type M = UpdateMessage;
    fn handle_message(&mut self, message: Self::M) -> Result<(), String> {
        match message {
            UpdateMessage::Destroy(id, index) => {
                self.instances.remove(&id);
                self.data_senders.remove(&id);
                self.remove_id_from_index(id, index);
            }
            UpdateMessage::ChangeMapping(id, old_idx, new_idx) => {
                try!(self.assign_id_to_index(id, new_idx));
                self.remove_id_from_index(id, old_idx);
            }
        }
        Ok(())
    }
}

impl HandleMessage for GtkVisualizerInstance {
    type M = DataMessage;
    fn handle_message(&mut self, message: Self::M) -> Result<(), String> {
        unimplemented!()
    }
}
