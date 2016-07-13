extern crate gtk;
extern crate gdk;
extern crate cairo;
use cairo::Context;
use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType, WindowPosition};
use gdk::WindowTypeHint;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Test Program");
    window.set_default_size(200, 200);
    window.set_wmclass("sildesktopwidget", "sildesktopwidget");
    window.set_type_hint(WindowTypeHint::Dock);
    window.set_decorated(false);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::Inhibit(false)
    });
    window.set_position(WindowPosition::Center);
    window.set_skip_pager_hint(true);
    window.set_skip_taskbar_hint(true);
    window.set_keep_below(true);
    window.show_all();
    gtk::main();
}
