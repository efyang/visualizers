use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use audio_updater::AudioUpdater;
use audio_devices::{get_devices, PaSourceInfo};
use audio_process::{AudioProcessor, FRAMES, AudioFrame};
use config::read_config;
use icon::default_status_icon;
use instance::GtkVisualizerInstance;
use gtk;
use gtk::{StatusIcon, Window, WindowType};

// NOTE: include the icon as bytes in the program
pub struct GtkVisualizerApp {
    // id needed for configs and title
    // when assigning: give current, then increment
    current_id_n: usize,
    pub instances: HashMap<usize, GtkVisualizerInstance>,
    icon: StatusIcon,
    program_continue: Arc<Mutex<bool>>, // whether the program whould continue, shared by app, all instances, and audio updater
}

impl GtkVisualizerApp {
    // Only call this once
    fn initialize() -> GtkVisualizerApp {
        gtk::init().expect("Failed to initialize GTK");
        // check if composited
        let test_window = Window::new(WindowType::Toplevel);
        let screen = WindowExt::get_screen(&test_window).unwrap();
        assert!(screen.is_composited(), "Gtk is not composited");
        drop(screen);
        drop(test_window);

        let program_continue = Arc::new(Mutex::new(true));

        // NOTE: rename get_devices to get_sources
        // initialize everything the audio updater needs
        let (default_source_name, sources) = get_devices().expect("Could not get any audio devices");
        let mut instances = HashMap::<usize, GtkVisualizerInstance>::new();
        let mut audio_processor_mappings = Vec::new();
        let (update_send, update_recv) = channel();
        let instance_configs = read_config();
        // read from configs and initialize instances here
        // ...
        // ...
        let mut current_data = Vec::new();
        // for (id, mut instance) in instances.iter_mut() {
        // let data_source = current_data.entry(instance.index())
        // .or_insert(Arc::new(Mutex::new(vec![vec![0f64; FRAMES]])));
        // instance.show_all();
        // }

        {
            let program_continue = program_continue.clone();
            let mut updater = AudioUpdater::new(&default_source_name, sources, audio_processor_mappings, update_recv, current_data, program_continue.clone());
            ::std::thread::spawn(move || {
                // startup the audio updater
                loop {
                    updater.iterate().unwrap_or_else(|e| {
                        println!("{}", e);
                        *program_continue.lock().unwrap() = true;
                    });
                }
            });
        }
        // initialize and set icon callbacks
        // ...
        // ...
        let icon = default_status_icon().unwrap();

        GtkVisualizerApp {
            current_id_n: instances.len(),
            instances: instances,
            icon: icon,
            program_continue: program_continue,
        };
        unimplemented!()
    }

    pub fn main_iteration(&mut self) -> Result<(), String> {
        // iterate instances
        for instance in self.instances.iter_mut().map(|(_, i)| i) {
            instance.iterate();
        }

        // run the actual gtk iteration
        gtk::main_iteration();
        Ok(())
    }
}

