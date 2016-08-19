use cairo::{Context, Operator};
use super::{BarData, CircleData, GradientData, Draw};

pub enum DrawingStyle {
    Bars(BarData),
    Circle(CircleData),
    Gradient(GradientData),
}

impl Default for DrawingStyle {
    fn default() -> Self {
        DrawingStyle::Bars(BarData::default())
    }
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

    fn draw_area(&self) -> (f64, f64) {
        match *self {
            DrawingStyle::Bars(ref bdata) => bdata.draw_area(),
            DrawingStyle::Circle(ref cdata) => cdata.draw_area(),
            DrawingStyle::Gradient(ref kgdata) => kgdata.draw_area(),
        }
    }
}
