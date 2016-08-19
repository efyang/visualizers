use std::cell::RefCell;
use super::traits::ConvertTo;
use super::super::drawing::{Color, DrawingStyle, BarData, GradientData, CircleData};

#[derive(Serialize, Deserialize)]
pub enum DrawingStyleConfig {
    Bars(BarDataConfig),
    Circle(CircleData),
    Gradient(GradientData),
}

impl Default for DrawingStyleConfig {
    fn default() -> Self {
        DrawingStyle::default().convert_to()
    }
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
pub struct BarDataConfig {
    pub double_sided: bool,
    pub num_bars: usize,
    pub split_audio_channels: bool,
    pub max_bar_pieces_vertical: usize,
    pub bar_piece_width: f64,
    pub bar_piece_height: f64,
    pub bar_piece_horizontal_spacing: f64,
    pub bar_piece_vertical_spacing: f64,
    pub draw_color: Color,
    pub bg_color: Color,
    pub top_padding: f64,
    pub bottom_padding: f64,
    pub left_padding: f64,
    pub right_padding: f64,
}

impl ConvertTo<BarDataConfig> for BarData {
    fn convert_to(&self) -> BarDataConfig {
        BarDataConfig {
            double_sided: self.double_sided,
            num_bars: self.num_bars,
            split_audio_channels: self.split_audio_channels,
            max_bar_pieces_vertical: self.max_bar_pieces_vertical,
            bar_piece_width: self.bar_piece_width,
            bar_piece_height: self.bar_piece_height,
            bar_piece_horizontal_spacing: self.bar_piece_horizontal_spacing,
            bar_piece_vertical_spacing: self.bar_piece_vertical_spacing,
            draw_color: self.draw_color.clone(),
            bg_color: self.bg_color.clone(),
            top_padding: self.top_padding,
            bottom_padding: self.bottom_padding,
            left_padding: self.left_padding,
            right_padding: self.right_padding,
        }
    }
}

impl ConvertTo<BarData> for BarDataConfig {
    fn convert_to(&self) -> BarData {
        BarData {
            double_sided: self.double_sided,
            num_bars: self.num_bars,
            split_audio_channels: self.split_audio_channels,
            max_bar_pieces_vertical: self.max_bar_pieces_vertical,
            bar_piece_width: self.bar_piece_width,
            bar_piece_height: self.bar_piece_height,
            bar_piece_horizontal_spacing: self.bar_piece_horizontal_spacing,
            bar_piece_vertical_spacing: self.bar_piece_vertical_spacing,
            draw_color: self.draw_color.clone(),
            bg_color: self.bg_color.clone(),
            top_padding: self.top_padding,
            bottom_padding: self.bottom_padding,
            left_padding: self.left_padding,
            right_padding: self.right_padding,
            peak_heights: ::std::cell::RefCell::new(vec![(0, 0.); self.num_bars]),
        }
    }
}
