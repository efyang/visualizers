use cairo::{Context, Operator};

pub mod color;
#[macro_use]
mod macros;
pub mod bar;
pub mod circle;
pub mod gradient;

pub use self::color::Color;
pub use self::bar::{BarData, BarDataConfig};
pub use self::circle::CircleData;
pub use self::gradient::GradientData;

pub trait Draw {
    fn draw(&self, context: &Context, data: &mut Vec<Vec<f64>>);
}

impl Draw for DrawingStyle {
    fn draw(&self, context: &Context, data: &mut Vec<Vec<f64>>) {
        context.set_operator(Operator::Source);
        match *self {
            DrawingStyle::Bars(ref bardata) => bardata.draw(context, data),
            DrawingStyle::Circle(ref circledata) => circledata.draw(context, data),
            DrawingStyle::Gradient(ref kuwodata) => kuwodata.draw(context, data),
        }
    }
}

impl Default for DrawingStyle {
    fn default() -> Self {
        DrawingStyle::Bars(BarData::default())
    }
}

pub trait GetDrawArea {
    fn draw_area(&self) -> (f64, f64);
}

impl GetDrawArea for DrawingStyle {
    // returns (w, h) of draw rect
    fn draw_area(&self) -> (f64, f64) {
        match *self {
            DrawingStyle::Bars(ref bdata) => bdata.draw_area(),
            DrawingStyle::Circle(ref cdata) => cdata.draw_area(),
            DrawingStyle::Gradient(ref kgdata) => kgdata.draw_area(),
        }
    }
}

pub enum DrawingStyle {
    Bars(BarData),
    Circle(CircleData),
    Gradient(GradientData),
}
