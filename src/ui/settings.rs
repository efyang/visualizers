use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition, SpinButton, Orientation, Align};
use shared_data::StateHolder;
use std::sync::mpsc::Sender;
use message::UpdateMessage;
use drawing::{BarData, CircleData, GradientData, DrawingStyle, Color};
use gdk;
use gtk;

trait ToGtkSettings {
    fn to_gtk_settings(&self) -> gtk::Box;
}

impl ToGtkSettings for StateHolder<DrawingStyle> {
    fn to_gtk_settings(&self) -> gtk::Box {
        let sbox = gtk::Box::new(Orientation::Vertical, 5);
        sbox.set_margin_top(10);
        match *self.borrow() {
            DrawingStyle::Bars(ref bdata) => {
                let double_sided = make_bool_changer!("Double Sided", double_sided, self, bars, bars_mut);
                // reasonable enough for now i guess
                let num_bars = make_usize_changer!("# Bars", 1, 8000, num_bars, self, bars, bars_mut);
                let split_audio_channels = make_bool_changer!("Split Audio Channels", split_audio_channels, self, bars, bars_mut);
                let max_bar_pieces_vertical = make_usize_changer!("Maximum Pieces per Bar", 1, 8000, max_bar_pieces_vertical, self, bars, bars_mut);
                let bar_piece_width = make_f64_changer!("Bar Piece Width", 1., 8000., bar_piece_width, self, bars, bars_mut);
                let bar_piece_height = make_f64_changer!("Bar Piece Height", 1., 8000., bar_piece_height, self, bars, bars_mut);
                let bar_piece_horizontal_spacing = make_f64_changer!("Bar Piece Horizontal Spacing", 0., 8000., bar_piece_horizontal_spacing, self, bars, bars_mut);
                let bar_piece_vertical_spacing = make_f64_changer!("Bar Piece Vertical Spacing", 0., 8000., bar_piece_vertical_spacing, self, bars, bars_mut);
                let draw_color = make_color_changer!("Bar Draw Color", draw_color, self, bars, bars_mut);
                let bg_color = make_color_changer!("Background Color", bg_color, self, bars, bars_mut);
                let top_padding = make_f64_changer!("Top Padding", 1., 8000., top_padding, self, bars, bars_mut);
                let bottom_padding = make_f64_changer!("Bottom Badding", 1., 8000., bottom_padding, self, bars, bars_mut);
                let left_padding = make_f64_changer!("Left Padding", 0., 8000., left_padding, self, bars, bars_mut);
                let right_padding = make_f64_changer!("Right Padding", 0., 8000., right_padding, self, bars, bars_mut);
                sbox.add(&double_sided);
                sbox.add(&num_bars);
                sbox.add(&split_audio_channels);
                sbox.add(&max_bar_pieces_vertical);
                sbox.add(&bar_piece_width);
                sbox.add(&bar_piece_height);
                sbox.add(&bar_piece_horizontal_spacing);
                sbox.add(&bar_piece_vertical_spacing);
                sbox.add(&draw_color);
                sbox.add(&bg_color);
                sbox.add(&top_padding);
                sbox.add(&bottom_padding);
                sbox.add(&left_padding);
                sbox.add(&right_padding);
            }
            DrawingStyle::Circle(ref cdata) => {

            }
            DrawingStyle::Gradient(ref gdata) => {

            }
        }
        sbox
    }
}

pub struct SettingsWindow {
    inner: Window,
}

impl SettingsWindow {
    pub fn new(id: usize,
               num_sources: usize,
               index: StateHolder<usize>,
               x: StateHolder<usize>,
               y: StateHolder<usize>,
               style: StateHolder<DrawingStyle>,
               update_sender: Sender<UpdateMessage>)
        -> Self {
            let window = Window::new(WindowType::Toplevel);
            window.set_position(WindowPosition::Center);
            let notebook = gtk::Notebook::new();
            window.add(&notebook);

            // let about_page =

            let w = gdk::screen_width();
            let h = gdk::screen_height();
            let x_control = new_dimension_box("X-Position", x.clone(), w as usize);
            let y_control = new_dimension_box("Y-Position", y.clone(), h as usize);
            let general_settings_page = gtk::Box::new(Orientation::Vertical, 5);
            general_settings_page.add(&x_control);
            general_settings_page.add(&y_control);
            {
                let bx = gtk::Box::new(Orientation::Horizontal, 0);
                let label = gtk::Label::new(Some("Audio Source Index"));
                label.set_halign(Align::Start);
                label.set_margin_left(10);
                let sb = SpinButton::new_with_range(0., (num_sources - 1) as f64, 1.);
                sb.set_value(*index.borrow() as f64);
                sb.connect_value_changed(move |sb| {
                    let newval = sb.get_value_as_int() as usize;
                    update_sender.send(UpdateMessage::ChangeMapping(id, *index.borrow(), newval)).unwrap();
                    *index.borrow_mut() = newval;
                });
                bx.add(&label);
                bx.add(&sb);
                general_settings_page.add(&bx);
            }
            add_tab(&notebook, "General", general_settings_page.upcast());

            let specific_page = style.to_gtk_settings();
            add_tab(&notebook, "Style-Specific", specific_page.upcast());

            SettingsWindow { inner: window }
        }

    pub fn show_all(&self) {
        self.inner.show_all();
    }
}


fn new_dimension_box(name: &str, dim_var: StateHolder<usize>, max: usize) -> gtk::Box {
    let bx = gtk::Box::new(Orientation::Horizontal, 0);
    let label = gtk::Label::new(Some(name));
    label.set_halign(Align::Start);
    label.set_margin_left(10);
    let sb = new_dimension_sb(dim_var, max);
    bx.add(&label);
    bx.add(&sb);
    bx
}

fn new_dimension_sb(dim_var: StateHolder<usize>, max: usize) -> SpinButton {
    let sb = SpinButton::new_with_range(-8000., max as f64, 1.);
    sb.set_value(*dim_var.borrow() as f64);
    sb.connect_value_changed(move |sb| {
        *dim_var.borrow_mut() = sb.get_value_as_int() as usize;
    });
    sb
}

fn add_tab(notebook: &gtk::Notebook, title: &str, widget: gtk::Widget) {
    let tab = gtk::Label::new(Some(title));
    tab.show_all();
    notebook.append_page(&widget, Some(&tab));
}
