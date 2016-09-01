use app::GtkVisualizerApp;

use super::traits::ConvertTo;
use super::instance::GtkVisualizerConfig;

#[derive(Serialize, Deserialize)]
struct ConfigStructure(Vec<GtkVisualizerConfig>);

impl ConvertTo<Vec<GtkVisualizerConfig>> for GtkVisualizerApp {
    fn convert_to(&self) -> Vec<GtkVisualizerConfig> {
        (*self.instances.borrow()).values().map(|v| v.convert_to()).collect()
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
