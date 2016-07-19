use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender, Receiver};

use cairo::Operator;
use gtk;
use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType, WindowPosition, StatusIcon};
use gdk::WindowTypeHint;

use audio_devices::{get_devices, PaSourceInfo};
use audio_process::AudioProcessor;
use config::{ConvertTo, GtkVisualizerConfig};
use drawing::*;
use messages::HandleMessage;

// NOTE: include the icon as bytes in the program

pub enum UpdateMessage {
    // id, index
    Destroy(usize, usize),
    // id, old index, new index
    ChangeMapping(usize, usize, usize),
}

pub type DataMessage = Vec<Vec<f64>>;

pub struct GtkVisualizerApp {
    // id needed for configs and title
    // when assigning: give current, then increment
    current_id_n: usize,
    pub instances: HashMap<usize, GtkVisualizerInstance>,
    default_source_name: String,
    pub sources: Vec<Option<PaSourceInfo>>,
    // multiple renderers per audio processor - have list of processors and map them to avoid
    // overuse of audio resources and conflicts
    // array w/ size of max index + 1
    // vec<usize> are the ids
    pub audio_processor_mappings: Vec<Option<(AudioProcessor, Vec<usize>)>>,
    icon: StatusIcon,
    // receiver for deletion messages
    msg_receiver: Receiver<UpdateMessage>,
    pub data_senders: HashMap<usize, Sender<DataMessage>>,
}

impl GtkVisualizerApp {
    // Only call this once
    fn initialize() -> GtkVisualizerApp {
        gtk::init().expect("Failed to initialize GTK");
        // make a test window to test whether composited
        let test_window = Window::new(WindowType::Toplevel);
        let screen = WindowExt::get_screen(&test_window).unwrap();
        assert!(screen.is_composited(), "Gtk is not composited");
        drop(screen);
        drop(test_window);

        let (default_source, devices) = get_devices().expect("Could not get any audio devices");
        let mut instances = HashMap::<usize, GtkVisualizerInstance>::new();
        let mut audio_processor_mappings = Vec::new();
        // read from configs here
        // ...
        // ...
        // let mut data_senders = Vec::new();
        let mut data_senders = HashMap::<usize, Sender<DataMessage>>::new();
        let (update_send, update_recv) = channel();
        for (id, mut instance) in instances.iter_mut() {
            let (data_send, data_recv) = channel();
            // data_senders.push(data_send);
            data_senders.insert(*id, data_send);
            instance.set_sender(update_send.clone());
            instance.set_receiver(data_recv);
            instance.show_all();
        }
        // initialize and set icon callbacks
        // ...
        // ...
        // use a channel for messages too? or just use update_send and add more message variants
        GtkVisualizerApp {
            current_id_n: instances.len(),
            instances: instances,
            default_source_name: default_source,
            sources: devices,
            audio_processor_mappings: audio_processor_mappings,
            icon: StatusIcon::new_from_file("icon.png"),
            msg_receiver: update_recv,
            data_senders: data_senders,
        };
        unimplemented!()
    }

    // Update source information
    fn update_sources(&mut self) -> Result<(), String> {
        let source_info = try!(get_devices());
        self.default_source_name = source_info.0;
        self.sources = source_info.1;
        Ok(())
    }

    fn default_source_index(&mut self) -> Option<usize> {
        for i in 0..self.sources.len() {
            if let Some(ref source_info) = self.sources[i] {
                if source_info.name == self.default_source_name {
                    return Some(i);
                }
            }
        }
        None
    }

    pub fn remove_id_from_index(&mut self, id: usize, index: usize) {
        let mut rm = false;
        if let Some((_, ref mut ids)) = self.audio_processor_mappings[index] {
            for i in 0..ids.len() {
                if ids[i] == id {
                    ids.swap_remove(i);
                }
            }
            if ids.len() == 0 {
                rm = true;
            }
        }
        // remove audio processor if no more instances are using it
        if rm {
            self.audio_processor_mappings[index] = None;
        }
    }

    pub fn assign_id_to_index(&mut self, id: usize, index: usize) -> Result<(), String> {
        if let Some((_, ref mut ids)) = self.audio_processor_mappings[index] {
            ids.push(id);
            return Ok(());
        }
        // if the processor doesn't exist, create it
        match AudioProcessor::new(self.sources.as_slice(), index) {
            Some(processor) => {
                self.audio_processor_mappings[index] = Some((processor, vec![id]));
                Ok(())
            }
            None => Err(format!("Could not set id {} to index {}", id, index)),
        }
    }

    pub fn main_iteration(&mut self) -> Result<(), String> {
        // check all messages
        match self.msg_receiver.try_recv() {
            Ok(m) => try!(self.handle_message(m)),
            Err(e) => return Err(format!("{}", e)),
        }

        // send data from audio processors
        for mapping in self.audio_processor_mappings.iter_mut() {
            if let Some((ref mut processor, ref ids)) = *mapping {
                let data = processor.get_data_frame();
                for id in ids {
                    if let Err(e) = self.data_senders[id].send(data.clone()) {
                        return Err(format!("{}", e));
                    }
                }
            }
        }

        // iterate instances
        for instance in self.instances.iter_mut().map(|(_, i)| i) {
            instance.iterate();
        }

        // run the actual gtk iteration
        gtk::main_iteration();
        Ok(())
    }
}

impl ConvertTo<Vec<GtkVisualizerConfig>> for GtkVisualizerApp {
    fn convert_to(&self) -> Vec<GtkVisualizerConfig> {
        self.instances.values().map(|v| v.convert_to()).collect()
    }
}

// how the hell do you update the drawing style when its getting used by 2 separate closures?
// ^ have connect_draw have a Receiver<DrawingStyle> and connect_clicked Sender<DrawingStyle>
pub struct GtkVisualizerInstance {
    id: usize,
    index: usize,
    window: Window,
    x_pos: usize,
    y_pos: usize,
    style: DrawingStyle,
    // both should always be Some after main app is initialized
    msg_sender: Option<Sender<UpdateMessage>>,
    data_receiver: Option<Receiver<DataMessage>>,
}

impl GtkVisualizerInstance {
    fn new(id: usize, x: usize, y: usize, index: usize) -> Self {
        let style = DrawingStyle::default();
        let window = Window::new(WindowType::Toplevel);
        let (draw_send, draw_recv) = channel::<DrawingStyle>();
        // IMPLEMENT REST
        GtkVisualizerInstance {
            id: id,
            index: index,
            window: window,
            x_pos: x,
            y_pos: y,
            style: style,
            msg_sender: None,
            data_receiver: None,
        };
        unimplemented!();
    }

    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn set_sender(&mut self, sender: Sender<UpdateMessage>) {
        self.msg_sender = Some(sender);
    }

    fn set_receiver(&mut self, receiver: Receiver<DataMessage>) {
        self.data_receiver = Some(receiver);
    }

    fn show_all(&self) {
        self.window.show_all();
    }

    fn iterate(&mut self) {
        unimplemented!()
    }
}

impl ConvertTo<GtkVisualizerConfig> for GtkVisualizerInstance {
    fn convert_to(&self) -> GtkVisualizerConfig {
        GtkVisualizerConfig {
            index: self.index,
            style: self.style.clone(),
            x_pos: self.x_pos,
            y_pos: self.y_pos,
        }
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
