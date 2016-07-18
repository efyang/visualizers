use gtk;
use cairo::Operator;
use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType, WindowPosition, StatusIcon};
use gdk::WindowTypeHint;

use audio_devices::{get_devices, PaSourceInfo};
use drawing::*;

pub struct VisualizerApp {
    // id needed for configs and title
    current_id_n: usize,
    instances: Vec<VisualizerInstance>,
    default_source_name: String,
    sources: Vec<Option<PaSourceInfo>>,
    // multiple renderers per processor - have list of processors and map them
    //audio_processors: Vec<>
    icon: StatusIcon,
}

impl VisualizerApp {
    // Only call this once
    fn initialize() -> VisualizerApp {
        VisualizerApp::initialize_with_instances(Vec::new())
    }

    fn initialize_with_instances(instances: Vec<VisualizerInstance>) -> VisualizerApp {
        // gtk::init
        // make a test window to test whether composited or find some other way
        unimplemented!()
    }

    // Update source information
    fn update_sources(&mut self) -> Result<(), String> {
        let source_info = try!(get_devices());
        self.default_source_name = source_info.0;
        self.sources = source_info.1;
        Ok(())
    }
}

pub struct VisualizerInstance {
    window: Window,
    x_pos: usize,
    y_pos: usize,
    style: DrawingStyle,
}

impl VisualizerInstance {
    fn new(id: usize, x: usize, y: usize) -> Self {
        let style = DrawingStyle::default();
        let window = Window::new(WindowType::Toplevel);
        // IMPLEMENT REST
        VisualizerInstance {
            window: window,
            x_pos: x,
            y_pos: y,
            style: style,
        }
    }
}

// temporary move - restructure later
pub fn run() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    transparent_window(WindowPosition::Mouse);

    let statusicon = StatusIcon::new_from_file("icon.png");
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
    } else {
        panic!("Cannot use non-composited screen");
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
