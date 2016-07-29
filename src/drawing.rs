use cairo::Context;
use data_helpers::shrink_by_averaging;

impl DrawingStyle {
    fn draw(&self, context: &Context, data: &[Vec<f64>]) {
        unimplemented!()
    }
}

impl Default for DrawingStyle {
    fn default() -> Self {
        DrawingStyle::Bars(BarData::default())
    }
}

impl Default for BarData {
    fn default() -> Self {
        BarData {
            double_sided: false,
            num_bars: 30,
            split_audio_channels: false,
            max_bar_pieces_vertical: 50,
            bar_piece_width: 4,
            bar_piece_height: 2,
            bar_piece_horizontal_spacing: 1,
            bar_piece_vertical_spacing: 1,
            draw_color: Color::black(),
            bg_color: Color::default_bg(),
            top_padding: 10,
            bottom_padding: 10,
            left_padding: 10,
            right_padding: 10,
        }
    }
}

impl Default for CircleData {
    fn default() -> Self {
        CircleData {
            split_audio_channels: false,
            min_radius: 10,
            max_radius: 50,
            draw_color: Color::black(),
            bg_color: Color::default_bg(),
            top_padding: 10,
            bottom_padding: 10,
            left_padding: 10,
            right_padding: 10,
        }
    }
}

impl Default for KuwoGradientData {
    fn default() -> Self {
        KuwoGradientData {
            split_audio_channels: true,
            height: 50,
            width: 300,
            bg_color: Color::default_bg(),
            gradient_start: Color::blue(),
            gradient_end: Color::yellow(),
            top_padding: 10,
            bottom_padding: 10,
            left_padding: 10,
            right_padding: 10,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DrawingStyle {
    Bars(BarData),
    Circle(CircleData),
    KuwoGradient(KuwoGradientData),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BarData {
    pub double_sided: bool,
    pub num_bars: usize,
    // draw channels seperately or average them into 1
    pub split_audio_channels: bool,
    pub max_bar_pieces_vertical: usize,
    pub bar_piece_width: usize,
    pub bar_piece_height: usize,
    pub bar_piece_horizontal_spacing: usize,
    pub bar_piece_vertical_spacing: usize,
    pub draw_color: Color,
    pub bg_color: Color,
    pub top_padding: usize,
    pub bottom_padding: usize,
    pub left_padding: usize,
    pub right_padding: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CircleData {
    pub split_audio_channels: bool,
    pub min_radius: usize,
    pub max_radius: usize,
    pub draw_color: Color,
    pub bg_color: Color,
    pub top_padding: usize,
    pub bottom_padding: usize,
    pub left_padding: usize,
    pub right_padding: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct KuwoGradientData {
    pub split_audio_channels: bool,
    pub height: usize,
    pub width: usize,
    pub bg_color: Color,
    pub gradient_start: Color,
    pub gradient_end: Color,
    pub top_padding: usize,
    pub bottom_padding: usize,
    pub left_padding: usize,
    pub right_padding: usize,
}

trait GetDrawArea {
    fn draw_area(&self) -> (usize, usize);
}

impl GetDrawArea for DrawingStyle {
    // returns (w, h) of draw rect
    fn draw_area(&self) -> (usize, usize) {
        match *self {
            DrawingStyle::Bars(ref bdata) => bdata.draw_area(),
            DrawingStyle::Circle(ref cdata) => cdata.draw_area(),
            DrawingStyle::KuwoGradient(ref kgdata) => kgdata.draw_area(),
        }
    }
}

impl GetDrawArea for BarData {
    fn draw_area(&self) -> (usize, usize) {
        let vert_mult;
        if self.double_sided {
            vert_mult = 2;
        } else {
            vert_mult = 1;
        }
        (self.bar_piece_width * self.num_bars +
         self.bar_piece_horizontal_spacing * (self.num_bars - 1) + self.right_padding +
         self.left_padding,
         vert_mult *
         (self.max_bar_pieces_vertical * self.bar_piece_height +
          self.bar_piece_vertical_spacing * (self.max_bar_pieces_vertical - 1)) +
         self.top_padding + self.bottom_padding)
    }
}

impl GetDrawArea for CircleData {
    fn draw_area(&self) -> (usize, usize) {
        let diameter = 2 * self.max_radius;
        (diameter + self.right_padding + self.left_padding,
         diameter + self.top_padding + self.bottom_padding)
    }
}

impl GetDrawArea for KuwoGradientData {
    fn draw_area(&self) -> (usize, usize) {
        (self.width + self.left_padding + self.right_padding,
         self.height + self.top_padding + self.bottom_padding)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Color(f64, f64, f64, f64);

impl Color {
    fn black() -> Self {
        Color(0., 0., 0., 1.)
    }

    fn default_bg() -> Self {
        Color(0.1, 0.1, 0.1, 0.4)
    }

    fn transparent() -> Self {
        Color(0., 0., 0., 0.)
    }

    // for gradient defaults
    // left side
    fn blue() -> Self {
        Color(0., 0., 1., 1.)
    }
    // right side
    fn yellow() -> Self {
        Color(1., 1., 0., 1.)
    }
}

macro_rules! call_rgba_fn {
    ($func:ident, $color:expr) => {
        if let (r,g,b,a) = $color {
            $func(r,g,b,a);
        }
    }
}
