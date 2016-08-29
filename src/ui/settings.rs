use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition, SpinButton, Orientation};
use shared_data::StateHolder;
use drawing::{BarData, CircleData, GradientData, DrawingStyle};
use gdk;
use gtk;

trait ToGtkSettings {
    fn to_gtk_settings(&self) -> gtk::Box;
}

macro_rules! make_bool_changer {
    ($name:expr, $fieldname:ident, $this_struct:ident) => {
        {
            let (bx, check) = new_bool_changer($name, (*$this_struct.borrow()).bars().unwrap().$fieldname);
            let bstruct = $this_struct.clone();
            check.connect_toggled(move |btn| {
                (*bstruct.borrow_mut()).bars_mut().unwrap().$fieldname = btn.get_active();
            });
            bx
        }
    }
}

impl ToGtkSettings for StateHolder<DrawingStyle> {
    fn to_gtk_settings(&self) -> gtk::Box {
        let sbox = gtk::Box::new(Orientation::Vertical, 0);
        match *self.borrow() {
            DrawingStyle::Bars(ref bdata) => {
                let dbside = make_bool_changer!("Double Sided", double_sided, self);
                sbox.add(&dbside);
            }
            DrawingStyle::Circle(ref cdata) => {

            }
            DrawingStyle::Gradient(ref gdata) => {

            }
        }
        sbox
    }
}

fn new_bool_changer(name: &str, value: bool) -> (gtk::Box, gtk::CheckButton) {
    let bx = gtk::Box::new(Orientation::Horizontal, 0);
    let label = gtk::Label::new(Some(name));
    let check = gtk::CheckButton::new();
    check.set_active(value);
    bx.add(&label);
    bx.add(&check);
    (bx, check)
}

pub struct SettingsWindow {
    inner: Window,
}

impl SettingsWindow {
    pub fn new(index: StateHolder<usize>,
               x: StateHolder<usize>,
               y: StateHolder<usize>,
               style: StateHolder<DrawingStyle>)
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
        let general_settings_page = gtk::Box::new(Orientation::Vertical, 0);
        general_settings_page.add(&x_control);
        general_settings_page.add(&y_control);
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
    let sb = new_dimension_sb(dim_var, max);
    bx.add(&label);
    bx.add(&sb);
    bx
}

fn new_dimension_sb(dim_var: StateHolder<usize>, max: usize) -> SpinButton {
    let sb = SpinButton::new_with_range(0., max as f64, 1.);
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
