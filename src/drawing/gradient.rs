use cairo::{Context, LinearGradient};
use super::{Draw, GetDrawArea};
use data_helpers::{scale, map_multiply, shrink_by_averaging};
use cairo::prelude::*;
use super::color::Color;

impl Draw for GradientData {
    fn draw(&self, context: &Context, data: &mut Vec<Vec<f64>>) {
        context.save();
        context.translate(self.left_padding, self.top_padding);
        let gradient = LinearGradient::new(0., 0., self.width, 0.);

        call_rgba_fn!(gradient, add_color_stop_rgba, 0., self.gradient_start);
        call_rgba_fn!(gradient, add_color_stop_rgba, 0.4, self.gradient_middle);
        call_rgba_fn!(gradient, add_color_stop_rgba, 0.6, self.gradient_middle);
        call_rgba_fn!(gradient, add_color_stop_rgba, 1., self.gradient_end);

        let half_height = (self.height - self.middle_line_height) / 2.;

        for mut datavec in data.iter_mut() {
            // scale(datavec); // replace this with a more absolute scale later on
            if self.width_desensitivity != 0 {
                shrink_by_averaging(&mut datavec,
                                    256 / (2usize.pow(self.width_desensitivity as u32)));
                // replace 256 with FRAMES
            }
            datavec.pop();
            {
                let tmplen = datavec.len();
                datavec[tmplen - 1] = 0.;
            }
            map_multiply(datavec, half_height);
            datavec.insert(0, half_height);
        }
        call_rgba_fn!(context, set_source_rgba, self.bg_color);
        context.paint();
        context.set_source(&gradient);
        context.rectangle(0., 0., self.width, self.height);
        context.fill();

        // draw the middle line;
        context.rectangle(0., half_height, self.width, self.middle_line_height);
        context.fill();

        context.translate(0., -self.middle_line_height / 2.);
        // bezier curve fun
        call_rgba_fn!(context, set_source_rgba, self.bg_color);
        let scale_x;
        let combined;
        let draw_half: Box<Fn()>;
        if self.split_audio_channels {
            scale_x = self.width / (data.len() * data[0].len()) as f64;
            combined = {
                let mut v = Vec::with_capacity(data.len() * data[0].len());
                v.extend(data[0].iter());
                v.extend(data[1].iter().rev());
                v
            };
            draw_half = Box::new(move || {
                let (p1, p2) = combined.split_at(combined.len() / 2);
                context.move_to(0., combined[0]);
                for (idx, chunk) in p1.chunks(4)
                    .enumerate()
                    .map(|(idx, c)| (idx * 4 + 1, c)) {
                    let (x1, y1) = ((idx + 1) as f64 * scale_x, (self.height / 2.) - chunk[1]);
                    let (x2, y2) = ((idx + 2) as f64 * scale_x, (self.height / 2.) - chunk[2]);
                    let (x3, y3) = ((idx + 3) as f64 * scale_x, (self.height / 2.) - chunk[3]);
                    context.curve_to(x1,
                                     y1,
                                     x2,
                                     y2,
                                     f64::min(self.width / 2., x3),
                                     f64::min(y3, half_height));
                }
                for (idx, chunk) in p2.chunks(4)
                    .enumerate()
                    .map(|(idx, c)| (idx * 4 + 1 + combined.len() / 2, c)) {
                    let (x1, y1) = ((idx + 1) as f64 * scale_x, (self.height / 2.) - chunk[1]);
                    let (x2, y2) = ((idx + 2) as f64 * scale_x, (self.height / 2.) - chunk[2]);
                    let (x3, y3) = ((idx + 3) as f64 * scale_x, (self.height / 2.) - chunk[3]);
                    if idx == combined.len() - 3 {
                        context.curve_to(x1, y1, x2, y2, f64::min(self.width, x3), half_height);
                    } else {
                        context.curve_to(x1,
                                         y1,
                                         x2,
                                         y2,
                                         f64::min(self.width, x3),
                                         f64::min(y3, half_height));
                    }
                }
                context.line_to(self.width, 0.);
                context.line_to(0., 0.);
                context.line_to(0., half_height);
                context.close_path();
                context.fill();
            });
        } else {
            scale_x = self.width / data[0].len() as f64;
            combined = {
                let mut v = Vec::with_capacity(data[0].len());
                for i in 0..data[0].len() {
                    v.push((data[0][i] + data[1][i]) / 2.);
                }
                v
            };
            draw_half = Box::new(move || {
                context.move_to(0., combined[0]);
                for (idx, chunk) in combined.chunks(4)
                    .enumerate()
                    .map(|(idx, c)| (idx * 4 + 1, c)) {
                    let (x1, y1) = ((idx + 1) as f64 * scale_x, (self.height / 2.) - chunk[1]);
                    let (x2, y2) = ((idx + 2) as f64 * scale_x, (self.height / 2.) - chunk[2]);
                    let (x3, y3) = ((idx + 3) as f64 * scale_x, (self.height / 2.) - chunk[3]);
                    context.curve_to(x1, y1, x2, y2, f64::min(self.width, x3), y3);
                }
                context.line_to(self.width, 0.);
                context.line_to(0., 0.);
                context.line_to(0., half_height);
                context.close_path();
                context.fill();
            });
        }
        // upper half
        draw_half();
        context.restore();
        context.save();
        context.scale(1., -1.);
        call_rgba_fn!(context, set_source_rgba, self.bg_color);
        context.translate(self.left_padding, -(self.top_padding + self.height));
        context.translate(0., -self.middle_line_height / 2.);
        // lower half
        draw_half();
        context.restore();
    }
}

impl Default for GradientData {
    fn default() -> Self {
        GradientData {
            split_audio_channels: true,
            height: 80.,
            width: 1200.,
            middle_line_height: 2.,
            bg_color: Color(0.1, 0.1, 0.1, 1.),
            gradient_start: Color::green(),
            gradient_middle: Color(1., 150. / 255., 80. / 255., 1.),
            gradient_end: Color::magenta(),
            width_desensitivity: 1,
            top_padding: 10.,
            bottom_padding: 10.,
            left_padding: 10.,
            right_padding: 10.,
        }
    }
}

impl GetDrawArea for GradientData {
    fn draw_area(&self) -> (f64, f64) {
        (self.width + self.left_padding + self.right_padding,
         self.height + self.top_padding + self.bottom_padding)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GradientData {
    pub split_audio_channels: bool,
    pub height: f64,
    pub width: f64,
    pub middle_line_height: f64,
    pub bg_color: Color,
    pub gradient_start: Color,
    pub gradient_middle: Color,
    pub gradient_end: Color,
    pub width_desensitivity: usize,
    pub top_padding: f64,
    pub bottom_padding: f64,
    pub left_padding: f64,
    pub right_padding: f64,
}
