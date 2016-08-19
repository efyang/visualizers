use super::{ConvertTo, GtkVisualizerConfig};
use super::super::app::GtkVisualizerApp;

#[derive(Serialize, Deserialize)]
struct ConfigStructure(Vec<GtkVisualizerConfig>);

impl ConvertTo<Vec<GtkVisualizerConfig>> for GtkVisualizerApp {
    fn convert_to(&self) -> Vec<GtkVisualizerConfig> {
        self.instances.values().map(|v| v.convert_to()).collect()
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
