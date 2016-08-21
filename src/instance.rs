use cairo::Operator;
use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition};
use gdk::WindowTypeHint;
use drawing::*;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use audio_input::AudioFrame;
use message::UpdateMessage;

// how the hell do you update the drawing style when its getting used by 2 separate closures?
// have instance have a Arc<Mutex<DrawingStyle>> and just mutate that
pub struct GtkVisualizerInstance {
    id: usize,
    pub index: usize,
    window: Window,
    pub x_pos: usize,
    pub y_pos: usize,
    pub style: Arc<Mutex<DrawingStyle>>,
    msg_sender: Sender<UpdateMessage>,
    data_sources: Vec<Option<Arc<Mutex<AudioFrame>>>>,
}

impl GtkVisualizerInstance {
    pub fn new(id: usize,
               x: usize,
               y: usize,
               index: usize,
               sources: &[Option<Arc<Mutex<AudioFrame>>>],
               update_sender: Sender<UpdateMessage>)
               -> Self {
        let style = DrawingStyle::default();
        Self::new_with_style(id, x, y, index, sources, style, update_sender)
    }

    pub fn new_with_style(id: usize,
                          x: usize,
                          y: usize,
                          index: usize,
                          sources: &[Option<Arc<Mutex<AudioFrame>>>],
                          style: DrawingStyle,
                          update_sender: Sender<UpdateMessage>)
                          -> Self {
        let window = Window::new(WindowType::Toplevel);
        let (draw_send, draw_recv) = channel::<DrawingStyle>();
        // IMPLEMENT REST
        GtkVisualizerInstance {
            id: id,
            index: index,
            window: window,
            x_pos: x,
            y_pos: y,
            style: Arc::new(Mutex::new(style)),
            msg_sender: update_sender,
            data_sources: sources.to_vec(),
        };
        unimplemented!();
    }


    fn id(&self) -> usize {
        self.id
    }

    fn index(&self) -> usize {
        self.index
    }

    pub fn show_all(&self) {
        self.window.show_all();
    }

    pub fn iterate(&mut self) {
        unimplemented!()
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
