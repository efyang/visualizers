use cairo::Context;
use super::color::Color;
use super::{Draw, GetDrawArea};
use data_helpers::{shrink_by_averaging, scale};

impl Draw for BarData {
    fn draw(&self, context: &Context, data: &mut Vec<Vec<f64>>) {
        context.save();
        let (_, totalheight) = self.draw_area();
        // draw the background
        call_rgba_fn!(context, set_source_rgba, self.bg_color);
        context.paint();

        call_rgba_fn!(context, set_source_rgba, self.draw_color);
        context.scale(1., -1.);
        context.translate(0., -totalheight);
        context.translate(self.left_padding, self.top_padding);
        let maxbarheight;
        if self.double_sided {
            maxbarheight = (totalheight - self.top_padding - self.bottom_padding)/2.;
        } else {
            maxbarheight = 0.;
        }
        let total_bars;
        let mut combined;
        if self.split_audio_channels {
            for datavec in data.iter_mut() {
                shrink_by_averaging(datavec, self.num_bars);
            }
            combined = Vec::with_capacity(self.num_bars * 2);
            combined.extend(data[0].iter());
            combined.extend(data[1].iter().rev());
            total_bars = self.num_bars * 2;
        } else {
            combined = Vec::with_capacity(self.num_bars);
            for i in 0..data[0].len() {
                let mut average = 0.;
                for datavec in data.iter() {
                    average += datavec[i];
                }
                average /= data.len() as f64;
                combined.push(average);
            }
            shrink_by_averaging(&mut combined, self.num_bars);
            total_bars = self.num_bars;
        }
        //scale(&mut combined);
        let draw_half: Box<Fn()> = Box::new(move || {
            for bar in 0..total_bars {
                let chunks = f64::min(self.max_bar_pieces_vertical as f64, combined[bar] * (self.max_bar_pieces_vertical as f64 - 1.)) as usize + 1;
                let x = bar as f64 * (self.bar_piece_width + self.bar_piece_horizontal_spacing);
                for i in 0..chunks {
                    context.rectangle(x,
                                      maxbarheight + i as f64 * (self.bar_piece_height + self.bar_piece_vertical_spacing),
                                      self.bar_piece_width,
                                      self.bar_piece_height);
                    context.fill();
                }
            }
        });

        draw_half();
        context.restore();
        if self.double_sided {
            context.save();
            context.translate(self.left_padding, self.top_padding);
            draw_half();
            context.restore();
        }
    }
}

impl Default for BarData {
    fn default() -> Self {
        BarData {
            double_sided: false,
            num_bars: 30,
            split_audio_channels: false,
            max_bar_pieces_vertical: 70,
            bar_piece_width: 16.,
            bar_piece_height: 4.,
            bar_piece_horizontal_spacing: 1.,
            bar_piece_vertical_spacing: 1.,
            draw_color: Color::black(),
            bg_color: Color::default_bg(),
            top_padding: 10.,
            bottom_padding: 10.,
            left_padding: 10.,
            right_padding: 10.,
        }
    }
}

impl GetDrawArea for BarData {
    fn draw_area(&self) -> (f64, f64) {
        let vert_mult;
        if self.double_sided {
            vert_mult = 2.;
        } else {
            vert_mult = 1.;
        }
        let hor_mult;
        if self.split_audio_channels {
            hor_mult = 2.;
        } else {
            hor_mult = 1.;
        }
        (hor_mult * (self.bar_piece_width * self.num_bars as f64 +
                     self.bar_piece_horizontal_spacing * (self.num_bars as f64 - 1.)) +
         self.right_padding +
         self.left_padding,
         vert_mult *
         (self.max_bar_pieces_vertical as f64 * self.bar_piece_height +
          self.bar_piece_vertical_spacing * (self.max_bar_pieces_vertical as f64 - 1.)) +
         self.top_padding + self.bottom_padding)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BarData {
    pub double_sided: bool,
    pub num_bars: usize,
    // draw channels seperately or average them into 1
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
