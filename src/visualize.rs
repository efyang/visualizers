use gtk;
use cairo::Operator;
use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType, WindowPosition};
use gdk::WindowTypeHint;
use std::thread::sleep;
use std::time::Duration;

// temporary move - restructure later
pub fn run() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    
    transparent_window(WindowPosition::Mouse);

    let statusicon = gtk::StatusIcon::new_from_file("icon.png");
    gtk::main();
}

fn transparent_window(pos: WindowPosition) {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Test Program");
    window.set_default_size(200, 200);
    window.set_wmclass("sildesktopwidget", "sildesktopwidget");
    window.set_type_hint(WindowTypeHint::Dock);
    window.set_decorated(false);
    window.set_position(pos);
    window.set_skip_pager_hint(true);
    window.set_skip_taskbar_hint(true);
    window.set_keep_below(true);
    window.set_app_paintable(true);
    let screen = WindowExt::get_screen(&window).unwrap();
    if screen.is_composited() {
        if let Some(alpha_screen) = screen.get_rgba_visual() {
            window.set_visual(Some(&alpha_screen));
        }
    }
   
    // initialize window drawing
    window.connect_draw(|_, context| {
        context.set_source_rgba(0., 0., 0., 0.5);
        context.set_operator(Operator::Source);
        context.paint();
        context.set_operator(Operator::Over);
        Inhibit(false)
    });

    window.connect_destroy(move |_| {
        transparent_window(pos);
    });
    window.show_all();
}
