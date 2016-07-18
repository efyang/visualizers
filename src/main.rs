extern crate cairo;
extern crate dft;
extern crate gtk;
extern crate gdk;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate libpulse_sys;
extern crate pa_simple;

mod audio_process;
mod audio_devices;
mod data_helpers;
mod visualize;
mod drawing;

use visualize::run;

fn main() {
    // run();
    println!("{:#?}", audio_devices::get_devices());
}
