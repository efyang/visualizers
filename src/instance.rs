use cairo::Operator;
use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition, Menu, MenuItem};
use gdk::WindowTypeHint;

use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

use audio_input::AudioFrame;
use drawing::*;
use gtk_helpers::is_right_click;
use message::UpdateMessage;
use shared_data::SharedData;

// how the hell do you update the drawing style when its getting used by 2 separate closures?
// have instance have a Arc<Mutex<DrawingStyle>> and just mutate that
pub struct GtkVisualizerInstance {
    id: usize,
    pub index: Arc<Mutex<usize>>,
    window: Window,
    pub x_pos: Arc<Mutex<usize>>,
    pub y_pos: Arc<Mutex<usize>>,
    pub style: Arc<Mutex<DrawingStyle>>,
    msg_sender: Sender<UpdateMessage>,
    data_sources: Vec<SharedData>,
}

macro_rules! clone_local {
    ( $( $x:ident ),* ) => {
        $(let $x = $x.clone();)*
    }
}

impl GtkVisualizerInstance {
    pub fn new(id: usize,
               x: usize,
               y: usize,
               index: usize,
               sources: &[SharedData],
               update_sender: Sender<UpdateMessage>)
               -> Self {
        let style = DrawingStyle::default();
        Self::new_with_style(id, x, y, index, sources, style, update_sender)
    }

    pub fn new_with_style(id: usize,
                          x: usize,
                          y: usize,
                          index: usize,
                          sources: &[SharedData],
                          style: DrawingStyle,
                          update_sender: Sender<UpdateMessage>)
                          -> Self {
        let window = Window::new(WindowType::Toplevel);

        window.set_title(&format!("Visualizers Instance {}", id));
        window.set_default_size(200, 200);
        window.move_(x as i32, y as i32);
        window.set_wmclass("sildesktopwidget", "sildesktopwidget");
        window.set_type_hint(WindowTypeHint::Dock);
        window.set_decorated(false);
        window.set_skip_pager_hint(true);
        window.set_skip_taskbar_hint(true);
        window.set_keep_below(true);
        window.set_app_paintable(true);

        let index = Arc::new(Mutex::new(index));
        let x_pos = Arc::new(Mutex::new(x));
        let y_pos = Arc::new(Mutex::new(y));
        let style = Arc::new(Mutex::new(style));
        let sources = sources.to_vec();

        // Setup draw operations
        {
            clone_local!(index, x_pos, y_pos, style, sources);
            window.connect_draw(move |window, context| {
                {
                    // resize to the needed draw size
                    let ref style = style.lock().unwrap();
                    let (width, height) = style.draw_area();
                    window.resize(width as i32, height as i32);
                    // get the source data
                    let ref item = sources[index.lock().unwrap().clone()];
                    // dunno whether to clone or not, just clone for now i guess
                    let mut unwrapped = item.lock().unwrap().clone();
                    match unwrapped {
                        Some(ref mut source) => {
                            // draw it
                            style.draw(context, source);
                        }
                        // Audio Processor not ready yet
                        None => {}
                    }
                }
                // move to a new position if any
                window.move_(*x_pos.lock().unwrap() as i32, *y_pos.lock().unwrap() as i32);

                Inhibit(false)
            });
        }

        // Setup right click context menu
        {
            clone_local!(index, x_pos, y_pos, style);
            window.connect_button_release_event(move |window, ebutton| {
                if is_right_click(ebutton) {
                    let (x, y) = ebutton.get_position();
                    let time = ebutton.get_time();
                    // create right click menu
                    let right_click_menu = Menu::new();
                    let menu_buttons = ["Close this instance", "Edit instance settings"];
                    for name in menu_buttons.iter() {
                        let item = MenuItem::new_with_label(name);
                        item.set_name(name);
                        right_click_menu.append(&item);
                    }
                    // null item workaround - unknown if still necessary
                    // let null_item = gtk::MenuItem::new_with_label("");
                    // null_item.set_name("None");
                    // right_click_menu.add(&null_item);
                    right_click_menu.set_active(menu_buttons.len() as u32);
                    right_click_menu.show_all();
                    // null_item.hide();
                    right_click_menu.popup_easy(3, time);

                    // right click menu callbacks
                    right_click_menu.connect_hide(move |this| {
                        if let Some(selection) = this.get_active() {
                            // get the index of the item
                            match &selection.get_name().unwrap() as &str {
                                "Close this instance" => {}
                                "Edit instance settings" => {}
                                _ => {}
                            }
                        }
                        this.destroy();
                    });
                }

                Inhibit(false)
            });
        }

        // IMPLEMENT REST
        GtkVisualizerInstance {
            id: id,
            index: index,
            window: window,
            x_pos: x_pos,
            y_pos: y_pos,
            style: style,
            msg_sender: update_sender,
            data_sources: sources,
        }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn index(&self) -> usize {
        *self.index.lock().unwrap()
    }

    pub fn show_all(&self) {
        self.window.show_all();
    }

    pub fn iterate(&mut self) {
        // add a custom timer or use gtk::timout_add?
        // self.window.queue_draw();
        // unimplemented!()
    }
}
