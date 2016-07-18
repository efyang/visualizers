use std::sync::mpsc::{Sender, Receiver};
use visualize::{GtkVisualizerApp, GtkVisualizerInstance, DataMessage, UpdateMessage};

trait HandleMessage {
    type M;
    fn handle_message(&mut self, message: &Self::M) -> Result<(), String>;
}

impl HandleMessage for GtkVisualizerApp {
    type M = UpdateMessage;
    fn handle_message(&mut self, message: &Self::M) -> Result<(), String> {
        unimplemented!()
    }
}

impl HandleMessage for GtkVisualizerInstance {
    type M = DataMessage;
    fn handle_message(&mut self, message: &Self::M) -> Result<(), String> {
        unimplemented!()
    }
}
