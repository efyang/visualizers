use gdk::{BUTTON3_MASK, EventButton, EventType};

pub fn is_right_click(ebutton: &EventButton) -> bool {
    if let EventType::ButtonRelease = ebutton.get_event_type() {
        let modtype = ebutton.get_state();
        modtype.contains(BUTTON3_MASK)
    } else {
        false
    }
}

#[macro_export]
macro_rules! make_bool_changer {
    ($name:expr, $fieldname:ident, $this_struct:ident, $unwrapper_fn:ident, $unwrapper_fn_mut:ident) => {
        {
            fn new_bool_changer(name: &str, value: bool) -> (gtk::Box, gtk::CheckButton) {
                let bx = gtk::Box::new(Orientation::Horizontal, 0);
                let label = gtk::Label::new(Some(name));
                label.set_halign(Align::Start);
                label.set_margin_left(10);
                let check = gtk::CheckButton::new();
                check.set_active(value);
                bx.add(&label);
                bx.add(&check);
                bx.set_homogeneous(true);
                (bx, check)
            }

            let (bx, check) = new_bool_changer($name, (*$this_struct.borrow()).$unwrapper_fn().unwrap().$fieldname);
            let bstruct = $this_struct.clone();
            check.connect_toggled(move |btn| {
                (*bstruct.borrow_mut()).$unwrapper_fn_mut().unwrap().$fieldname = btn.get_active();
            });
            bx
        }
    }
}

#[macro_export]
macro_rules! make_usize_changer {
    ($name:expr, $min:expr, $max:expr, $fieldname:ident, $this_struct:ident, $unwrapper_fn:ident, $unwrapper_fn_mut:ident) => {
        {
            fn new_usize_changer(name: &str, value: usize, min: usize, max: usize) -> (gtk::Box, gtk::SpinButton) {
                let bx = gtk::Box::new(Orientation::Horizontal, 0);
                let label = gtk::Label::new(Some(name));
                label.set_halign(Align::Start);
                label.set_margin_left(10);
                let check = gtk::SpinButton::new_with_range(min as f64, max as f64, 1.);
                check.set_value(value as f64);
                bx.add(&label);
                bx.add(&check);
                bx.set_homogeneous(true);
                (bx, check)
            }

            let (bx, spin) = new_usize_changer($name, (*$this_struct.borrow()).$unwrapper_fn().unwrap().$fieldname, $min, $max);
            let bstruct = $this_struct.clone();
            spin.connect_value_changed(move |sb| {
                (*bstruct.borrow_mut()).$unwrapper_fn_mut().unwrap().$fieldname = sb.get_value_as_int() as usize;
            });
            bx
        }
    }
}

#[macro_export]
macro_rules! make_f64_changer {
    ($name:expr, $min:expr, $max:expr, $fieldname:ident, $this_struct:ident, $unwrapper_fn:ident, $unwrapper_fn_mut:ident) => {
        {
            fn new_f64_changer(name: &str, value: f64, min: f64, max: f64) -> (gtk::Box, gtk::SpinButton) {
                let bx = gtk::Box::new(Orientation::Horizontal, 0);
                let label = gtk::Label::new(Some(name));
                label.set_halign(Align::Start);
                label.set_margin_left(10);
                let check = gtk::SpinButton::new_with_range(min, max, 1.);
                check.set_value(value);
                bx.add(&label);
                bx.add(&check);
                bx.set_homogeneous(true);
                (bx, check)
            }

            let (bx, spin) = new_f64_changer($name, (*$this_struct.borrow()).$unwrapper_fn().unwrap().$fieldname, $min, $max);
            let bstruct = $this_struct.clone();
            spin.connect_value_changed(move |sb| {
                (*bstruct.borrow_mut()).$unwrapper_fn_mut().unwrap().$fieldname = sb.get_value();
            });
            bx
        }
    }
}

#[macro_export]
macro_rules! make_color_changer {
    ($name:expr, $fieldname:ident, $this_struct:ident, $unwrapper_fn:ident, $unwrapper_fn_mut:ident) => {
        {
            fn new_color_changer(name: &str, value: Color) -> (gtk::Box, gtk::ColorButton) {
                let bx = gtk::Box::new(Orientation::Horizontal, 0);
                let label = gtk::Label::new(Some(name));
                label.set_halign(Align::Start);
                label.set_margin_left(10);
                let colorchange = gtk::ColorButton::new_with_rgba(&value.into());
                colorchange.set_use_alpha(true);
                bx.add(&label);
                bx.add(&colorchange);
                bx.set_homogeneous(true);
                (bx, colorchange)
            }

            let (bx, colorbtn) = new_color_changer($name, (*$this_struct.borrow()).$unwrapper_fn().unwrap().$fieldname.clone());
            let bstruct = $this_struct.clone();
            colorbtn.connect_color_set(move |btn| {
                (*bstruct.borrow_mut()).$unwrapper_fn_mut().unwrap().$fieldname = btn.get_rgba().into();
            });
            bx
        }
    }
}
