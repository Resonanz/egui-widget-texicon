use egui::{ImageSource, Label, Margin, Response, RichText, Rounding, Sense, Ui, Vec2, Widget};

//
// ======================================================================
// ======================================================================
// ======================================================================
//

#[derive(Clone)]
pub struct Config<'a> {
    img: Option<&'a ImageSource<'a>>, // Wrap in Option for impl Default
    text: &'a str,
    frame_size: Vec2,
    icon_size: Vec2,
    text_size: f32,
    inner_margin: Margin,
    outer_margin: Margin,
    rounding: Rounding,
    icon_text_gap: f32,
    // frame_outline_width: f32,
    // frame_outline_color: Color32,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            img: None,
            text: "",
            frame_size: Default::default(),
            icon_size: Default::default(),
            text_size: Default::default(),
            inner_margin: Default::default(),
            outer_margin: Default::default(),
            rounding: Default::default(),
            icon_text_gap: Default::default(),
        }
    }
}

//
// ======================================================================
// ======================================================================
// ======================================================================
//

// Builder starts here
pub struct ConfigBuilder<'a> {
    img: Option<&'a ImageSource<'a>>,
    text: &'a str,
    frame_size: Vec2,
    icon_size: Vec2,
    text_size: f32,
    inner_margin: Margin,
    outer_margin: Margin,
    rounding: Rounding,
    icon_text_gap: f32,
    // frame_outline_width: f32,
    // frame_outline_color: Color32,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new(img: Option<&'a ImageSource<'a>>) -> Self {
        ConfigBuilder {
            img,
            text: "Default text",
            frame_size: Vec2 { x: 40.0, y: 60.0 },
            icon_size: Vec2 { x: 40.0, y: 40.0 },
            text_size: 16.0,
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            rounding: Rounding::same(0.0),
            icon_text_gap: 0.0,
            // frame_outline_width: 1.0,
            // frame_outline_color: Color32::TRANSPARENT,
        }
    }

    pub fn text(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }

    pub fn frame_size(mut self, frame_size: Vec2) -> Self {
        self.frame_size = frame_size;
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

    pub fn inner_margin(mut self, inner_margin: Margin) -> Self {
        self.inner_margin = inner_margin;
        self
    }

    pub fn outer_margin(mut self, outer_margin: Margin) -> Self {
        self.outer_margin = outer_margin;
        self
    }

    pub fn rounding(mut self, rounding: Rounding) -> Self {
        self.rounding = rounding;
        self
    }

    pub fn icon_text_space(mut self, icon_text_gap: f32) -> Self {
        self.icon_text_gap = icon_text_gap;
        self
    }

    // pub fn frame_outline_width(mut self, frame_outline_width: f32) -> Self {
    //     self.frame_outline_width = frame_outline_width;
    //     self
    // }

    // pub fn frame_outline_color(mut self, frame_outline_color: Color32) -> Self {
    //     self.frame_outline_color = frame_outline_color;
    //     self
    // }

    pub fn build(self) -> Config<'a> {
        Config {
            img: self.img,
            text: self.text,
            frame_size: self.frame_size,
            icon_size: self.icon_size,
            text_size: self.text_size,
            inner_margin: self.inner_margin,
            outer_margin: self.outer_margin,
            rounding: self.rounding,
            icon_text_gap: self.icon_text_gap,
            // frame_outline_width: self.frame_outline_width,
            // frame_outline_color: self.frame_outline_color,
        }
    }
}
//
// ======================================================================
// ======================================================================
// ======================================================================
//

///
/// A number of different methods are used
/// to share information between your app
/// and the widget.
///
/// 1. A ```Config``` struct is used to configure
///    the properties of the widget.
/// 2. A ```SidebarTexicon``` struct is used to
///    share state information between the widget
///    and the main application.
///
///
#[derive(Default, PartialEq)]
pub enum TexiMouseState {
    #[default]
    None,
    Clicked,
    Hovering,
}

#[derive(Default, PartialEq)]
pub enum TexiColorState {
    #[default]
    Dim,
    On,
    Highlight,
}

// WARNING: Complicated
//
// The purpose of this struct's contents is to
// create portals between the main code and this
// widget code. It does this using references and
// pointers to communicate via e.g. bools and enums.
//
// This struct is instantiated in the calling crate.
//
//
pub struct TexiItem<'a> {
    pub texi_mouse: TexiMouseState,
    pub texi_color: TexiColorState,
    pub config: Config<'a>,
}

impl<'a> Default for TexiItem<'a> {
    fn default() -> Self {
        Self {
            texi_mouse: TexiMouseState::None, // Initialized value
            texi_color: TexiColorState::Dim,  // Initialized value
            config: Default::default(),
        }
    }
}

impl<'a> TexiItem<'a> {
    pub fn new(config: Config<'a>) -> Self {
        Self {
            texi_mouse: TexiMouseState::None, // Initialized value
            texi_color: TexiColorState::Dim,  // Initialized value
            config,                           // config calls ConfigBuilder to populate
        }
    }
}

//
// ======================================================================
// ======================================================================
// ======================================================================
//

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Texicon<'a> {
    mouse_state: &'a mut TexiMouseState,
    color_state: &'a mut TexiColorState,
    config: &'a Config<'a>,
}

impl<'a> Texicon<'a> {
    // TexiItem struct received and destructured,
    // providing references and pointer access
    // to mouse and color items inside the vector.
    pub fn new(texi: &'a mut TexiItem) -> Self {
        Self {
            mouse_state: &mut texi.texi_mouse,
            color_state: &mut texi.texi_color,
            config: &mut texi.config,
        }
    }

    pub fn get_tet(&self) -> u32 {
        123
    }
}

impl<'a> Widget for Texicon<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Define the frame and allocate its inner UI
        let response = egui::Frame::default()
            .inner_margin(self.config.inner_margin)
            .outer_margin(self.config.outer_margin)
            .rounding(self.config.rounding)
            .fill(ui.style().visuals.extreme_bg_color)
            // .stroke(egui::Stroke {
            //     width: self.config.frame_outline_width,
            //     color: self.config.frame_outline_color.gamma_multiply(0.5),
            // })
            .show(ui, |ui| {
                // Set the minimum size of
                // the ui (that is, the frame)
                ui.set_min_size(self.config.frame_size);
                ui.set_max_size(self.config.frame_size); // Layout the icon and text vertically with some spacing

                // Set colors for text and icon
                let tint_color = match *self.color_state {
                    TexiColorState::Dim => ui.style().visuals.weak_text_color(),
                    TexiColorState::On => ui.style().visuals.text_color(),
                    TexiColorState::Highlight => ui.style().visuals.warn_fg_color,
                };

                //
                ui.allocate_ui_with_layout(
                    self.config.frame_size,
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        // Add icon
                        let icon_response = ui
                            .add_sized(
                                self.config.icon_size,
                                egui::Image::new(self.config.img.unwrap().to_owned())
                                    .tint(tint_color), // Adjust color if necessary
                            )
                            .interact(Sense::click());

                        // Add some vertical spacing
                        ui.add_space(self.config.icon_text_gap);

                        // Add text
                        let text_response = ui
                            .add(
                                Label::new(
                                    RichText::new(self.config.text)
                                        .color(tint_color)
                                        .size(self.config.text_size),
                                )
                                .selectable(false),
                            )
                            .interact(Sense::click());

                        // Update state depending upon response
                        if icon_response.clicked {
                            *self.mouse_state = TexiMouseState::Clicked
                        } else if icon_response.contains_pointer() {
                            *self.mouse_state = TexiMouseState::Hovering
                        } else {
                            *self.mouse_state = TexiMouseState::None
                        }

                        *self.mouse_state = if icon_response.clicked || text_response.clicked {
                            TexiMouseState::Clicked
                        } else if icon_response.contains_pointer()
                            || text_response.contains_pointer()
                        {
                            TexiMouseState::Hovering
                        } else {
                            TexiMouseState::None
                        }
                    },
                );
            })
            .response;

        response
    }
}
