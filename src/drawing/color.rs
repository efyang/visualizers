use gdk_sys::GdkRGBA;

#[derive(Clone, Serialize, Deserialize)]
pub struct Color(pub f64, pub f64, pub f64, pub f64);

impl Color {
    pub fn black() -> Self {
        Color(0., 0., 0., 1.)
    }

    pub fn default_bg() -> Self {
        Color(0.1, 0.1, 0.1, 0.2)
    }

    // for gradient defaults
    // left side
    pub fn green() -> Self {
        Color(0., 1., 0., 1.)
    }
    // right side
    pub fn magenta() -> Self {
        Color(1., 0., 1., 1.)
    }
}

impl Into<GdkRGBA> for Color {
    fn into(self) -> GdkRGBA {
        GdkRGBA {
            red: self.0,
            green: self.1,
            blue: self.2,
            alpha: self.3,
        }
    }
}

impl Into<Color> for GdkRGBA {
    fn into(self) -> Color {
        Color(self.red, self.green, self.blue, self.alpha)
    }
}
