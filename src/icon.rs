use gtk::StatusIcon;
use gdk_pixbuf::PixbufLoader;

pub fn default_status_icon() -> Result<StatusIcon, String> {
    let data = include_bytes!("../resources/icon.png");
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
        },
        None => Err("Failed to load status icon.".to_string())
    }
}
