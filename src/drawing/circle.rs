use cairo::Context;
use super::{Draw, GetDrawArea};
use super::color::Color;
use data_helpers::{shrink_by_averaging, scale};

impl Draw for CircleData {
    fn draw(&self, context: &Context, data: &mut Vec<Vec<f64>>) {
        context.save();
        context.translate(self.left_padding, self.top_padding);
        // draw the background
        call_rgba_fn!(context, set_source_rgba, self.bg_color);
        context.paint();

        call_rgba_fn!(context, set_source_rgba, self.draw_color);

        let max_increase = self.max_radius - self.min_radius;

        let mut combined;
        if self.split_audio_channels {
            for datavec in data.iter_mut() {
                shrink_by_averaging(datavec, 90);
            }
            combined = Vec::with_capacity(180);
            combined.extend(data[0].iter());
            combined.extend(data[1].iter().rev());
        } else {
            combined = Vec::with_capacity(256);
            for i in 0..data[0].len() {
                let mut average = 0.;
                for datavec in data.iter() {
                    average += datavec[i];
                }
                average /= data.len() as f64;
                combined.push(average);
            }
            shrink_by_averaging(&mut combined, 180);
        }
        //scale(&mut combined);

        let mut points = Vec::new();
        let rotation_angle = match self.rotation {
            None => 0.,
            Some(angle) => angle,
        };

        for (angle, percentage) in combined.into_iter()
            .enumerate()
            .map(|(a, p)| (a as f64 * 2., p)) {
            let radian = to_radians(angle as f64 + rotation_angle);
            let actual_radius = self.min_radius + (max_increase * percentage);
            let x = actual_radius * radian.cos();
            let y = actual_radius * radian.sin();
            points.push((x, y));
        }

        context.translate(self.max_radius, self.max_radius);
        context.move_to(points[0].0, points[0].1);
        for &(x, y) in points.iter().skip(1) {
            context.line_to(x, y);
        }
        context.close_path();
        context.stroke();
        context.restore();
    }
}

const RADIANS_PER_ANGLE: f64 = ::std::f64::consts::PI / 180.;
fn to_radians(angle: f64) -> f64 {
    angle * RADIANS_PER_ANGLE
}

impl Default for CircleData {
    fn default() -> Self {
        CircleData {
            split_audio_channels: false,
            min_radius: 70.,
            max_radius: 200.,
            draw_color: Color::black(),
            bg_color: Color::default_bg(),
            rotation: None,
            top_padding: 10.,
            bottom_padding: 10.,
            left_padding: 10.,
            right_padding: 10.,
        }
    }
}

impl GetDrawArea for CircleData {
    fn draw_area(&self) -> (f64, f64) {
        let diameter = 2. * self.max_radius;
        (diameter + self.right_padding + self.left_padding,
         diameter + self.top_padding + self.bottom_padding)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CircleData {
    pub split_audio_channels: bool,
    pub min_radius: f64,
    pub max_radius: f64,
    pub draw_color: Color,
    pub bg_color: Color,
    pub rotation: Option<f64>, // degrees to rotate
    pub top_padding: f64,
    pub bottom_padding: f64,
    pub left_padding: f64,
    pub right_padding: f64,
}
