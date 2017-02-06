#![allow(dead_code, unused_variables, unused_imports)]
#![feature(plugin, concat_idents)]
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
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate time;

#[macro_use]
mod macros;
mod app;
mod audio_input;
mod autostart;
mod config;
mod data_helpers;
mod drawing;
mod ui;
mod instance;
mod lockfile;
mod message;
mod shared_data;

use app::GtkVisualizerApp;

fn main() {
    let mut app = GtkVisualizerApp::initialize();
    loop {
        if let Err(e) = app.main_iteration() {
            println!("{}", e);
            break;
        }
    }
}
