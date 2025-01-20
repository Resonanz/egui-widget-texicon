/// Builder-patterned code for setting parameters for a Texicon widget.
///
/// This code is separate from the widget code. The struct fields are
/// filled in using the Builder and the struct is finally returned to
/// the caller. The initialized struct is then passed into the widget
/// whereupon the configuration data in the struct is utilized.
use egui::{Color32, ImageSource, Margin, Rounding, Vec2};

#[derive(Clone)]
pub struct Config {
    pub img: ImageSource<'static>,
    pub text: String,
    pub frame_size: Vec2,
    pub frame_outline_width: f32,
    pub frame_outline_color: Color32,
    pub frame_inner_margin: Margin,
    pub frame_outer_margin: Margin,
    pub frame_rounding: Rounding,
    pub icon_size: Vec2,
    pub text_size: f32,
    pub icon_text_gap: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            img: egui::include_image!("../assets/pics/gear.svg"),
            text: String::from("No text defined"),
            frame_size: Default::default(),
            frame_outline_width: Default::default(),
            frame_outline_color: Default::default(),
            frame_inner_margin: Default::default(),
            frame_outer_margin: Default::default(),
            frame_rounding: Default::default(),
            icon_size: Default::default(),
            text_size: Default::default(),
            icon_text_gap: Default::default(),
        }
    }
}

//
// ======================================================================
// ======================================================================
// ======================================================================
//

// This struct holds all
// Texicon configuration.
pub struct ConfigBuilder {
    // img: Option<&'a ImageSource<'a>>,
    img: ImageSource<'static>,
    text: String,
    frame_size: Vec2,
    frame_outline_width: f32,
    frame_outline_color: Color32,
    frame_inner_margin: Margin,
    frame_outer_margin: Margin,
    frame_rounding: Rounding,
    icon_size: Vec2,
    text_size: f32,
    icon_text_gap: f32,
}

impl ConfigBuilder {
    // pub fn new(img: Option<&'a ImageSource<'a>>) -> Self {
    pub fn new() -> Self {
        ConfigBuilder {
            // Default values
            img: egui::include_image!("../assets/pics/gear.svg"),
            text: String::from("No text defined"),
            frame_size: Vec2 { x: 40.0, y: 60.0 },
            frame_outline_width: 4.0,
            frame_outline_color: Color32::TRANSPARENT,
            frame_inner_margin: Margin::same(0.0),
            frame_outer_margin: Margin::same(0.0),
            frame_rounding: Rounding::same(0.0),
            icon_size: Vec2 { x: 40.0, y: 40.0 },
            text_size: 16.0,
            icon_text_gap: 0.0,
        }
    }

    pub fn img(mut self, img: ImageSource<'static>) -> Self {
        self.img = img;
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn frame_size(mut self, frame_size: Vec2) -> Self {
        self.frame_size = frame_size;
        self
    }

    pub fn frame_outline_width(mut self, frame_outline_width: f32) -> Self {
        self.frame_outline_width = frame_outline_width;
        self
    }

    pub fn frame_outline_color(mut self, frame_outline_color: Color32) -> Self {
        self.frame_outline_color = frame_outline_color;
        self
    }

    pub fn frame_inner_margin(mut self, frame_inner_margin: Margin) -> Self {
        self.frame_inner_margin = frame_inner_margin;
        self
    }

    pub fn frame_outer_margin(mut self, frame_outer_margin: Margin) -> Self {
        self.frame_outer_margin = frame_outer_margin;
        self
    }

    pub fn frame_rounding(mut self, frame_rounding: Rounding) -> Self {
        self.frame_rounding = frame_rounding;
        self
    }

    pub fn icon_size(mut self, icon_size: Vec2) -> Self {
        self.icon_size = icon_size;
        self
    }

    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    pub fn icon_text_space(mut self, icon_text_gap: f32) -> Self {
        self.icon_text_gap = icon_text_gap;
        self
    }

    pub fn build(self) -> Config {
        Config {
            img: self.img,
            text: self.text,
            frame_size: self.frame_size,
            frame_outline_width: self.frame_outline_width,
            frame_outline_color: self.frame_outline_color,
            frame_inner_margin: self.frame_inner_margin,
            frame_outer_margin: self.frame_outer_margin,
            frame_rounding: self.frame_rounding,
            icon_size: self.icon_size,
            text_size: self.text_size,
            icon_text_gap: self.icon_text_gap,
        }
    }
}
