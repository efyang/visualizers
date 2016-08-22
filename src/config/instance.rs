use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;

use super::traits::ConvertTo;
use super::drawingstyle::DrawingStyleConfig;

use audio_input::AudioFrame;
use instance::GtkVisualizerInstance;
use message::UpdateMessage;
use shared_data::SharedData;

#[derive(Serialize, Deserialize)]
pub struct GtkVisualizerConfig {
    pub index: usize,
    pub style: DrawingStyleConfig,
    pub x_pos: usize,
    pub y_pos: usize,
}

impl Default for GtkVisualizerConfig {
    fn default() -> Self {
        GtkVisualizerConfig {
            index: 0,
            style: DrawingStyleConfig::default(),
            x_pos: 0,
            y_pos: 0,
        }
    }
}

impl GtkVisualizerConfig {
    pub fn to_instance(self,
                   id: usize,
                   sources: &[SharedData],
                   update_sender: Sender<UpdateMessage>)
                   -> GtkVisualizerInstance {
        GtkVisualizerInstance::new_with_style(id,
                                              self.x_pos,
                                              self.y_pos,
                                              self.index,
                                              sources,
                                              self.style.convert_to(),
                                              update_sender)
    }
}

impl ConvertTo<GtkVisualizerConfig> for GtkVisualizerInstance {
    fn convert_to(&self) -> GtkVisualizerConfig {
        GtkVisualizerConfig {
            index: (*(self.index.lock().unwrap())),
            style: (*(self.style.lock().unwrap())).convert_to(),
            x_pos: (*(self.x_pos.lock().unwrap())),
            y_pos: (*(self.y_pos.lock().unwrap())),
        }
    }
}
