use cairo::Operator;
use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition, Menu, MenuItem};
use gtk;
use gdk::WindowTypeHint;
use time::precise_time_ns;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

use audio_input::AudioFrame;
use drawing::*;
use ui::{is_right_click, SettingsWindow};
use message::UpdateMessage;
use shared_data::{SharedData, StateHolder};

// make this changeable in program settings later on: Arc<Mutex> for each instance
const DRAW_UPDATE_TIME: u64 = 1000_000_000; // ns or 500 ms

// how the hell do you update the drawing style when its getting used by 2 separate closures?
// have instance have a Arc<Mutex<DrawingStyle>> and just mutate that
pub struct GtkVisualizerInstance {
    id: usize,
    pub index: StateHolder<usize>,
    window: Window,
    pub x_pos: StateHolder<usize>,
    pub y_pos: StateHolder<usize>,
    pub style: StateHolder<DrawingStyle>,
    msg_sender: Sender<UpdateMessage>,
    data_sources: Vec<SharedData>,
    last_drawn: u64,
    instance_continue: StateHolder<bool>,
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
        update_sender.send(UpdateMessage::Add(id, index)).unwrap();
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
        let screen = WindowExt::get_screen(&window).unwrap();
        if screen.is_composited() {
            if let Some(alpha_screen) = screen.get_rgba_visual() {
                window.set_visual(Some(&alpha_screen));
            }
        } else {
            panic!("Cannot use non-composited screen");
        }

        let index = Rc::new(RefCell::new(index));
        let x_pos = Rc::new(RefCell::new(x));
        let y_pos = Rc::new(RefCell::new(y));
        let style = Rc::new(RefCell::new(style));
        let instance_continue = Rc::new(RefCell::new(true));
        let sources = sources.to_vec();

        // Setup draw operations
        {
            clone_local!(index, x_pos, y_pos, style, sources);
            window.connect_draw(move |window, context| {
                {
                    // resize to the needed draw size
                    let style = &*style.borrow();
                    let (width, height) = style.draw_area();
                    window.resize(width as i32, height as i32);
                    // get the source data
                    let item = &sources[*index.borrow()];
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
                window.move_(*x_pos.borrow() as i32, *y_pos.borrow() as i32);

                Inhibit(false)
            });
        }

        // Setup right click context menu
        // workaround for weird double popups (BUG)
        let already_spawned_popup = Rc::new(RefCell::new(false));
        {
            let num_sources = sources.len();
            clone_local!(index,
                         x_pos,
                         y_pos,
                         style,
                         already_spawned_popup,
                         update_sender,
                         instance_continue);
            window.connect_button_release_event(move |window, ebutton| {
                if is_right_click(ebutton) {
                    if !*already_spawned_popup.borrow() {
                        *already_spawned_popup.borrow_mut() = true;

                        let time = ebutton.get_time();
                        // create right click menu
                        let right_click_menu = Menu::new();
                        let menu_buttons = ["Close this instance", "Edit instance settings"];
                        for name in menu_buttons.iter() {
                            let item = MenuItem::new_with_label(name);
                            item.set_name(name);
                            right_click_menu.append(&item);
                        }
                        // null item workaround - to detect if no buttons clicked
                        let null_item = MenuItem::new_with_label("");
                        null_item.set_name("None");
                        right_click_menu.add(&null_item);
                        right_click_menu.set_active(menu_buttons.len() as u32);
                        right_click_menu.show_all();
                        null_item.hide();
                        right_click_menu.popup_easy(3, time);

                        // right click menu callbacks
                        let already_spawned_popup = already_spawned_popup.clone();
                        {
                            clone_local!(index, x_pos, y_pos, style, update_sender, instance_continue);
                            right_click_menu.connect_hide(move |this| {
                                clone_local!(index, x_pos, y_pos, style, update_sender, instance_continue);
                                if let Some(selection) = this.get_active() {
                                    // get the index of the item
                                    match &selection.get_name().unwrap() as &str {
                                        "Close this instance" => {
                                            *instance_continue.borrow_mut() = false;
                                            update_sender.send(UpdateMessage::Destroy(id, *index.borrow())).unwrap();
                                        }
                                        "Edit instance settings" => {
                                            let settings = SettingsWindow::new(id,
                                                                               num_sources,
                                                                               index,
                                                                               x_pos,
                                                                               y_pos,
                                                                               style,
                                                                               update_sender);
                                            settings.show_all();
                                        }
                                        _ => {}
                                    }
                                }
                                this.destroy();
                                *already_spawned_popup.borrow_mut() = false;
                            });
                        }
                    }
                }

                Inhibit(false)
            });
        }
        window.show_all();

        GtkVisualizerInstance {
            id: id,
            index: index,
            window: window,
            x_pos: x_pos,
            y_pos: y_pos,
            style: style,
            msg_sender: update_sender,
            data_sources: sources,
            last_drawn: precise_time_ns(),
            instance_continue: instance_continue,
        }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn index(&self) -> usize {
        *self.index.borrow()
    }

    pub fn iterate(&mut self) -> bool {
        // add a custom timer or use gtk::timout_add?
        let time_now = precise_time_ns();
        if time_now > self.last_drawn + DRAW_UPDATE_TIME {
            self.window.queue_draw();
        }
        *self.instance_continue.borrow()
    }
}

impl Drop for GtkVisualizerInstance {
    fn drop(&mut self) {
        self.window.destroy();
    }
}
