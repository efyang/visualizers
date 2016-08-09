use std::fs::{File, create_dir_all};
use std::path::PathBuf;
use std::io;

use serde_yaml::{from_reader, to_writer};
use app_dirs::{get_app_dir, AppDirType};
use drawing::{DrawingStyle, BarData, CircleData, GradientData, BarDataConfig};

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

#[derive(Serialize, Deserialize)]
struct ConfigStructure(Vec<GtkVisualizerConfig>);

#[derive(Serialize, Deserialize)]
pub enum DrawingStyleConfig {
    Bars(BarDataConfig),
    Circle(CircleData),
    Gradient(GradientData),
}

impl ConvertTo<DrawingStyleConfig> for DrawingStyle {
    fn convert_to(&self) -> DrawingStyleConfig {
        match *self {
            DrawingStyle::Bars(ref bdata) => DrawingStyleConfig::Bars(bdata.convert_to()),
            DrawingStyle::Circle(ref cdata) => DrawingStyleConfig::Circle(cdata.clone()),
            DrawingStyle::Gradient(ref kgdata) => DrawingStyleConfig::Gradient(kgdata.clone()),
        }
    }
}

impl ConvertTo<DrawingStyle> for DrawingStyleConfig {
    fn convert_to(&self) -> DrawingStyle {
        match *self {
            DrawingStyleConfig::Bars(ref bdata) => DrawingStyle::Bars(bdata.convert_to()),
            DrawingStyleConfig::Circle(ref cdata) => DrawingStyle::Circle(cdata.clone()),
            DrawingStyleConfig::Gradient(ref kgdata) => DrawingStyle::Gradient(kgdata.clone()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GtkVisualizerConfig {
    pub index: usize,
    pub style: DrawingStyleConfig,
    pub x_pos: usize,
    pub y_pos: usize,
}

pub trait ConvertTo<T> {
    fn convert_to(&self) -> T;
}

fn read_config() -> io::Result<Vec<GtkVisualizerConfig>> {
    if !CONFIG_PATH.exists() {
        Err(io::Error::new(io::ErrorKind::NotFound, "Config does not exist"))
    } else {
        let config = try!(File::open(CONFIG_PATH.as_path()));
        match from_reader(config) {
            Ok(read) => Ok(read),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("Error on reading config: {}", e))),
        }
    }
}

fn write_config(config: Vec<GtkVisualizerConfig>) -> io::Result<()> {
    try!(create_dir_all(CONFIG_PATH.parent().unwrap()));
    let mut config_out = try!(File::create(CONFIG_PATH.as_path()));
    if let Err(e) = to_writer(&mut config_out, &config) {
        Err(io::Error::new(io::ErrorKind::Other, format!("Error on writing config: {}", e)))
    } else {
        Ok(())
    }
}

impl Into<ConfigStructure> for Vec<GtkVisualizerConfig> {
    fn into(self) -> ConfigStructure {
        ConfigStructure(self)
    }
}

impl Into<Vec<GtkVisualizerConfig>> for ConfigStructure {
    fn into(self) -> Vec<GtkVisualizerConfig> {
        let ConfigStructure(inner) = self;
        inner
    }
}
