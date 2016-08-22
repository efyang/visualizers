use std::sync::{Arc, Mutex};
use audio_input::AudioFrame;

pub type SharedData = Arc<Mutex<Option<AudioFrame>>>;
