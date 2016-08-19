pub mod color;
#[macro_use]
mod macros;
pub mod bar;
pub mod circle;
pub mod gradient;
mod style;

use cairo::{Context, Operator};

pub use self::color::Color;
pub use self::bar::BarData;
pub use self::circle::CircleData;
pub use self::gradient::GradientData;
pub use self::style::DrawingStyle;

// maybe combine these two traits?
pub trait Draw {
    fn draw(&self, context: &Context, data: &mut Vec<Vec<f64>>);
    fn draw_area(&self) -> (f64, f64);
}
