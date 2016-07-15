extern crate gtk;
extern crate gdk;
extern crate cairo;
extern crate dft;
extern crate pulse_simple;

mod audio_process;
mod visualize;
mod data_helpers;

use visualize::run;

fn main() {
    run();
}
