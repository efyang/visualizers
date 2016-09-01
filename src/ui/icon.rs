use std::collections::HashMap;
use std::sync::mpsc::Sender;
use instance::GtkVisualizerInstance;
use shared_data::{ContinueState, StateHolder, SharedData};
use gtk::{StatusIcon, Menu, MenuItem};
use gdk_pixbuf::PixbufLoader;
use super::helpers::is_right_click;
use message::UpdateMessage;
use config::GtkVisualizerConfig;
use gtk::prelude::*;

pub fn default_status_icon() -> Result<StatusIcon, String> {
    let data = include_bytes!("../../resources/icon.png");
    let loader = PixbufLoader::new();
    if let Err(e) = loader.loader_write(data) {
        return Err(format!("{}", e));
    }
    match loader.get_pixbuf() {
        Some(pb) => {
            if let Err(e) = loader.close() {
                return Err(format!("{}", e));
            }
            Ok(StatusIcon::new_from_pixbuf(&pb))
        }
        None => Err("Failed to load status icon.".to_string()),
    }
}

pub fn set_icon_callbacks(icon: &StatusIcon,
                          id_counter: StateHolder<usize>,
                          instances: StateHolder<HashMap<usize, GtkVisualizerInstance>>,
                          data: Vec<SharedData>,
                          update_sender: Sender<UpdateMessage>,
                          default_index: usize,
                          program_continue: ContinueState) {
    icon.set_tooltip_text("Visualizers");
    icon.connect_button_release_event(move |icon, ebtn| {
        if is_right_click(ebtn) {
            let time = ebtn.get_time();
            let right_click_menu = Menu::new();
            let menu_buttons = ["New Instance", "Quit"];
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
            {
                clone_local!(program_continue, id_counter, update_sender, data, instances);
                right_click_menu.connect_hide(move |this| {
                    if let Some(selection) = this.get_active() {
                        match &selection.get_name().unwrap() as &str {
                            "New Instance" => {
                                {
                                    let newid = *id_counter.borrow();
                                    (*instances.borrow_mut())
                                        .insert(newid,
                                                GtkVisualizerConfig {
                                                        index: default_index,
                                                        ..GtkVisualizerConfig::default()
                                                    }
                                                    .to_instance(newid,
                                                                 &data,
                                                                 update_sender.clone()));
                                }
                                *id_counter.borrow_mut() += 1;
                            }
                            "Quit" => {
                                program_continue.set(false);
                            }
                            _ => {}
                        }
                    }
                });
            }
        }
        true
    });
}
