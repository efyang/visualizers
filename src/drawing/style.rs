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

macro_rules! make_unwrapper {
    ($name:ident, $used:ident, $ignore1:ident, $ignore2:ident, $out:ty) => {
        pub fn $name(&self) -> Option<&$out> {
            match *self {
                DrawingStyle::$used(ref data) => Some(data),
                DrawingStyle::$ignore1(_) => None,
                DrawingStyle::$ignore2(_) => None,
            }
        }
    };
    (m, $name:ident, $used:ident, $ignore1:ident, $ignore2:ident, $out:ty) => {
        pub fn $name(&mut self) -> Option<&mut $out> {
            match *self {
                DrawingStyle::$used(ref mut data) => Some(data),
                DrawingStyle::$ignore1(_) => None,
                DrawingStyle::$ignore2(_) => None,
            }
        }
    };
}

impl DrawingStyle {
    make_unwrapper!(bars, Bars, Circle, Gradient, BarData);
    make_unwrapper!(m, bars_mut, Bars, Circle, Gradient, BarData);
    make_unwrapper!(circle, Circle, Bars, Gradient, CircleData);
    make_unwrapper!(m, circle_mut, Circle, Bars, Gradient, CircleData);
    make_unwrapper!(gradient, Gradient, Circle, Bars, GradientData);
    make_unwrapper!(m, gradient_mut, Gradient, Circle, Bars, GradientData);
}
