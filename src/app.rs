use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};

use gtk;
use gtk::prelude::*;
use gtk::{StatusIcon, Window, WindowType};

use audio_input::AudioUpdater;
use audio_input::{get_sources, PaSourceInfo};
use audio_input::{AudioProcessor, FRAMES, AudioFrame};
use shared_data::ContinueState;
use config::read_config;
use icon::default_status_icon;
use instance::GtkVisualizerInstance;
use message::UpdateMessage;

// NOTE: include the icon as bytes in the program
pub struct GtkVisualizerApp {
    // id needed for configs and title
    // when assigning: give current, then increment
    current_id_n: usize,
    pub instances: HashMap<usize, GtkVisualizerInstance>,
    icon: StatusIcon,
    program_continue: ContinueState, /* whether the program whould continue, shared by app, all instances, and audio updater */
}

impl GtkVisualizerApp {
    // Only call this once
    pub fn initialize() -> GtkVisualizerApp {
        gtk::init().expect("Failed to initialize GTK");
        // check if composited
        let test_window = Window::new(WindowType::Toplevel);
        let screen = WindowExt::get_screen(&test_window).unwrap();
        assert!(screen.is_composited(), "Gtk is not composited");
        drop(screen);
        drop(test_window);

        let program_continue = ContinueState::new(true);

        // initialize everything the audio updater needs
        let (default_source_name, sources) = get_sources()
            .expect("Could not get any audio devices");
        let default_source_index = default_source_index(&default_source_name, &sources).unwrap();
        let num_sources = sources.len();
        let mut instances = HashMap::<usize, GtkVisualizerInstance>::new();
        let (update_send, update_recv) = channel();
        let current_data = vec![Arc::new(Mutex::new(None)); num_sources];
        let audio_processor_mappings = (0..num_sources).map(|_| None).collect();

        let instance_configs = read_config().unwrap();
        let mut instance_id = 0;
        for config in instance_configs {
            update_send.send(UpdateMessage::Add(instance_id, config.index)).unwrap();
            // NOTE: temporary
            update_send.send(UpdateMessage::ChangeMapping(instance_id, config.index, default_source_index)).unwrap();
            let instance = config.to_instance(instance_id, &current_data, update_send.clone());
            instance.show_all();
            instances.insert(instance_id, instance);
            instance_id += 1;
        }

        {
            let program_continue = program_continue.clone();
            let mut updater = AudioUpdater::new(&default_source_name,
                                                sources,
                                                audio_processor_mappings,
                                                update_recv,
                                                current_data,
                                                program_continue.clone());
            ::std::thread::spawn(move || {
                // startup the audio updater
                loop {
                    updater.iterate().unwrap_or_else(|e| {
                        panic!("{}", e);
                        //*program_continue.lock().unwrap() = false;
                    });
                }
            });
        }

        // initialize and set icon callbacks
        // ...
        // ...
        //unimplemented!()
        let icon = default_status_icon().unwrap();

        GtkVisualizerApp {
            current_id_n: instance_id,
            instances: instances,
            icon: icon,
            program_continue: program_continue,
        }
    }

    pub fn main_iteration(&mut self) -> Result<(), String> {
        // iterate instances
        for instance in self.instances.iter_mut().map(|(_, i)| i) {
            instance.iterate();
        }

        // run the actual gtk iteration
        if !self.program_continue.get() {
            Err("Program ended".to_string())
        } else {
            //while gtk::events_pending()
            // ^ this kills the cpu
                gtk::main_iteration();
            Ok(())
        }
    }
}

fn default_source_index(default_source_name: &str,
                        sources: &Vec<Option<PaSourceInfo>>)
                        -> Option<usize> {
    for i in 0..sources.len() {
        if let Some(ref source_info) = sources[i] {
            if &source_info.name == default_source_name {
                return Some(i);
            }
        }
    }
    None
}
