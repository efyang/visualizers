use gdk::{BUTTON3_MASK, EventButton, EventType};

pub fn is_right_click(ebutton: &EventButton) -> bool {
    if let EventType::ButtonRelease = ebutton.get_event_type() {
        let modtype = ebutton.get_state();
        modtype.contains(BUTTON3_MASK)
    } else {
        false
    }
}
