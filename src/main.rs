#![allow(dead_code, unused_variables, unused_imports)]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate app_dirs;
extern crate cairo;
extern crate dft;
extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gdk_sys;
extern crate glib;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate libpulse_sys;
extern crate pa_simple;
extern crate serde;
extern crate serde_yaml;

mod app;
mod audio_input;
mod config;
mod data_helpers;
mod drawing;
mod icon;
mod instance;
mod lockfile;
mod message;

use app::GtkVisualizerApp;

use std::sync::{Arc, Mutex};
use drawing::*;
use audio_input::{PaSourceInfo, AudioProcessor};
use gtk::prelude::*;
use gtk::{Window, WindowType};

const WIDTH: usize = 800;
const HEIGHT: usize = 150;

lazy_static! {
    static ref DATA: Mutex<Vec<Vec<f64>>> = Mutex::new(vec![vec![0f64; 256]; 2]);
}

fn main() {
    // run();
    gtk::init().unwrap();
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Test Program");
    window.set_default_size(WIDTH as i32, HEIGHT as i32);
    window.set_app_paintable(true);
    let screen = WindowExt::get_screen(&window).unwrap();
    if screen.is_composited() {
        if let Some(alpha_screen) = screen.get_rgba_visual() {
            window.set_visual(Some(&alpha_screen));
        }
    } else {
        panic!("Cannot use non-composited screen");
    }

    let drawstyle = DrawingStyle::default();
    window.connect_draw(move |window, context| {
        let (width, height) = drawstyle.draw_area();
        window.resize(width as i32, height as i32);
        let mut data = DATA.lock().unwrap().clone();
        drawstyle.draw(&context, &mut data);
        gtk::Inhibit(false)
    });

    window.connect_destroy(|_| gtk::main_quit());
    window.show_all();

    // VERY IMPORTANT
    gtk::timeout_add(30, move || {
        window.queue_draw();
        gtk::Continue(true)
    });

    ::std::thread::spawn(move || {
        let (default_source_name, sources) = audio_input::get_devices().unwrap();
        let default_source_index = default_source_index(default_source_name, &sources).unwrap();
        let mut processor = AudioProcessor::new(&sources, default_source_index).unwrap();
        loop {
            // https://github.com/astro/rust-pulse-simple/issues/2
            // (1000ms / ms_sleep) * FRAMES >= SAMPLE_RATE (44100 in this case)
            //::std::thread::sleep_ms(5); -- desyncs audio - dont use it
            let newvec = processor.get_data_frame();
            *DATA.lock().unwrap() = newvec;
        }
    });

    gtk::main();
}

fn default_source_index(default_source_name: String,
                        sources: &Vec<Option<PaSourceInfo>>)
                        -> Option<usize> {
    for i in 0..sources.len() {
        if let Some(ref source_info) = sources[i] {
            if source_info.name == default_source_name {
                return Some(i);
            }
        }
    }
    None
}
