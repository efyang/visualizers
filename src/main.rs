#![allow(dead_code, unused_variables, unused_imports)]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate app_dirs;
extern crate cairo;
extern crate dft;
extern crate gtk;
extern crate gdk;
extern crate gdk_sys;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate libpulse_sys;
extern crate pa_simple;
extern crate serde_yaml;

mod app;
mod audio_process;
mod audio_devices;
mod config;
mod data_helpers;
mod drawing;
mod instance;

use app::GtkVisualizerApp;

fn main() {
    // run();
    println!("{:#?}", audio_devices::get_devices());
}
