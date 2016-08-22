mod traits;
mod drawingstyle;
mod app;
mod instance;

use std::fs::{File, create_dir_all};
use std::path::PathBuf;
use std::io;

use serde_yaml::{from_reader, to_writer};
use app_dirs::{get_app_dir, AppDirType};
use app::GtkVisualizerApp;

use self::traits::ConvertTo;
use self::drawingstyle::DrawingStyleConfig;
use self::instance::GtkVisualizerConfig;

const CONFIG_NAME: &'static str = "visualizers.yml";
const CONFIG_DIR: &'static str = "visualizers";

lazy_static! {
    // NOTE: panic should be changed to a better alternative
    static ref CONFIG_PATH: PathBuf = {
        match get_app_dir(AppDirType::UserConfig) {
            Ok(p) => p.join(CONFIG_DIR).join(CONFIG_NAME),
            Err(_) => panic!("Could not get config file"),
        }
    };
}

pub fn read_config() -> io::Result<Vec<GtkVisualizerConfig>> {
    if !CONFIG_PATH.exists() {
        let def_vec = vec![GtkVisualizerConfig::default()];
        try!(write_config(&def_vec));
        Ok(def_vec)
    } else {
        let config = try!(File::open(CONFIG_PATH.as_path()));
        match from_reader(config) {
            Ok(read) => Ok(read),
            Err(e) => {
                Err(io::Error::new(io::ErrorKind::Other,
                                   format!("Error on reading config: {}", e)))
            }
        }
    }
}

pub fn write_config(config: &Vec<GtkVisualizerConfig>) -> io::Result<()> {
    let mut config_out = try!(create_config_file());
    if let Err(e) = to_writer(&mut config_out, &config) {
        Err(io::Error::new(io::ErrorKind::Other,
                           format!("Error on writing config: {}", e)))
    } else {
        Ok(())
    }
}

fn create_config_file() -> io::Result<File> {
    try!(create_dir_all(CONFIG_PATH.parent().unwrap()));
    File::create(CONFIG_PATH.as_path())
}
