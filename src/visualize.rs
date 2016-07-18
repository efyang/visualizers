use std::sync::mpsc::{channel, Sender, Receiver};

use cairo::Operator;
use gtk;
use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType, WindowPosition, StatusIcon};
use gdk::WindowTypeHint;

use audio_devices::{get_devices, PaSourceInfo};
use audio_process::AudioProcessor;
use drawing::*;

// NOTE: include the icon as bytes in the program

pub enum UpdateMessage {
    // id
    Destroy(usize),
    // id, new index
    ChangeMapping(usize, usize),
}

pub type DataMessage = Vec<f32>;

pub struct GtkVisualizerApp {
    // id needed for configs and title
    // when assigning: give current, then increment
    current_id_n: usize,
    instances: Vec<GtkVisualizerInstance>,
    default_source_name: String,
    sources: Vec<Option<PaSourceInfo>>,
    // multiple renderers per audio processor - have list of processors and map them to avoid
    // overuse of audio resources and conflicts
    // array w/ size of max index + 1
    // vec<usize> are the ids
    audio_processor_mappings: Vec<Option<(AudioProcessor, Vec<usize>)>>,
    icon: StatusIcon,
    // receiver for deletion messages
    msg_receiver: Receiver<UpdateMessage>,
    data_senders: Vec<Sender<DataMessage>>,
}

impl GtkVisualizerApp {
    // Only call this once
    fn initialize() -> GtkVisualizerApp {
        gtk::init().expect("Failed to initialize GTK");
        // make a test window to test whether composited
        assert!(gtk_is_composited(), "Gtk is not composited");
        let (default_source, devices) = get_devices().expect("Could not get any audio devices");
        let mut instances = Vec::<GtkVisualizerInstance>::new();
        let mut audio_processor_mappings = Vec::new();
        // read from configs here
        // ...
        // ...
        let mut data_senders = Vec::new();
        let (update_send, update_recv) = channel();
        for mut instance in instances.iter_mut() {
            let (data_send, data_recv) = channel();
            data_senders.push(data_send);
            instance.set_sender(update_send.clone());
            instance.set_receiver(data_recv);
        }

        GtkVisualizerApp {
            current_id_n: instances.len(),
            instances: instances,
            default_source_name: default_source,
            sources: devices,
            audio_processor_mappings: audio_processor_mappings,
            icon: StatusIcon::new_from_file("icon.png"),
            msg_receiver: update_recv,
            data_senders: data_senders,
        }
    }

    // Update source information
    fn update_sources(&mut self) -> Result<(), String> {
        let source_info = try!(get_devices());
        self.default_source_name = source_info.0;
        self.sources = source_info.1;
        Ok(())
    }
}

// must be run only after gtk is initialized
fn gtk_is_composited() -> bool {
    assert!(gtk::is_initialized());
    let test_window = Window::new(WindowType::Toplevel);
    let screen = WindowExt::get_screen(&test_window).unwrap();
    screen.is_composited()
}

// how the hell do you update the drawing style when its getting used by 2 separate closures?
// ^ have connect_draw have a Receiver<DrawingStyle> and connect_clicked Sender<DrawingStyle>
pub struct GtkVisualizerInstance {
    window: Window,
    x_pos: usize,
    y_pos: usize,
    style: DrawingStyle,
    // both should always be Some after main app is initialized
    msg_sender: Option<Sender<UpdateMessage>>,
    data_receiver: Option<Receiver<DataMessage>>,
}

impl GtkVisualizerInstance {
    fn new(id: usize, x: usize, y: usize) -> Self {
        let style = DrawingStyle::default();
        let window = Window::new(WindowType::Toplevel);
        // IMPLEMENT REST
        GtkVisualizerInstance {
            window: window,
            x_pos: x,
            y_pos: y,
            style: style,
            msg_sender: None,
            data_receiver: None,
        }
        ;unimplemented!()
    }

    fn set_sender(&mut self, sender: Sender<UpdateMessage>) {
        self.msg_sender = Some(sender);
    }

    fn set_receiver(&mut self, receiver: Receiver<DataMessage>) {
        self.data_receiver = Some(receiver);
    }
}

// restructure later
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
    // make the visualizer instance in here
    // clone the sender to use for connect_destroy, move rest to connect_draw
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
