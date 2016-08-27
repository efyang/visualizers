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
extern crate time;

mod app;
mod audio_input;
mod config;
mod data_helpers;
mod drawing;
mod gtk_helpers;
mod gtk_settings;
mod icon;
mod instance;
mod lockfile;
mod message;
mod shared_data;

use app::GtkVisualizerApp;

fn main() {
    let mut app = GtkVisualizerApp::initialize();
    loop {
        app.main_iteration().unwrap();
    }
}
