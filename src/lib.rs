use egui::{
    Color32, ImageSource, Label, Margin, Response, RichText, Rounding, Sense, Ui, Vec2, Widget,
};

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
    frame_outline_width: f32,
    frame_outline_color: Color32,
    frame_inner_margin: Margin,
    frame_outer_margin: Margin,
    frame_rounding: Rounding,
    icon_size: Vec2,
    text_size: f32,
    icon_text_gap: f32,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            img: None,
            text: "",
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
pub struct ConfigBuilder<'a> {
    img: Option<&'a ImageSource<'a>>,
    text: &'a str,
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

impl<'a> ConfigBuilder<'a> {
    pub fn new(img: Option<&'a ImageSource<'a>>) -> Self {
        ConfigBuilder {
            // Default values
            img,
            text: "Default text",
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

    pub fn text(mut self, text: &'a str) -> Self {
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

    pub fn build(self) -> Config<'a> {
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

//
// ======================================================================
// ======================================================================
// ======================================================================
//

// This struct in instantiated
// from/in the non-widget code.
// The struct is this passed
// back here into new() as a
// reference to create a portal
// between the two codebases.
#[derive(Default)]
pub struct TexiconSharedVars {
    pub mouse_state: TexiMouseState,
    pub color_state: TexiColorState,
}

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Texicon<'a> {
    pub mouse_state_ref: &'a mut TexiMouseState,
    color_state_ref: &'a mut TexiColorState,
    config_ref: &'a Config<'a>,
}

impl<'a> Texicon<'a> {
    // TexiconSharedVars and Config structs
    // are passed into here and destructured.
    // This provides references for pointer
    // access so that data can be shared by
    // reference betwee the two codebases.
    // Further, Config allows access to the
    // ConfigBuilder generated config data.
    pub fn new(texi_shared_vars: &'a mut TexiconSharedVars, config: &'a Config<'a>) -> Self {
        Self {
            mouse_state_ref: &mut texi_shared_vars.mouse_state,
            color_state_ref: &mut texi_shared_vars.color_state,
            config_ref: &config,
        }
    }
}

impl<'a> Widget for Texicon<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Define the frame and allocate its inner UI
        let response = egui::Frame::default()
            .inner_margin(self.config_ref.frame_inner_margin)
            .outer_margin(self.config_ref.frame_outer_margin)
            .rounding(self.config_ref.frame_rounding)
            .fill(ui.style().visuals.extreme_bg_color)
            .stroke(egui::Stroke {
                width: self.config_ref.frame_outline_width,
                color: self.config_ref.frame_outline_color.gamma_multiply(0.5),
            })
            .show(ui, |ui| {
                // Set the minimum size of
                // the ui (that is, the frame)
                ui.set_min_size(self.config_ref.frame_size);
                ui.set_max_size(self.config_ref.frame_size); // Layout the icon and text vertically with some spacing

                // Set colors for text and icon
                let tint_color = match *self.color_state_ref {
                    TexiColorState::Dim => ui.style().visuals.weak_text_color(),
                    TexiColorState::On => ui.style().visuals.text_color(),
                    TexiColorState::Highlight => ui.style().visuals.warn_fg_color,
                };

                //
                ui.allocate_ui_with_layout(
                    self.config_ref.frame_size,
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        // Add icon
                        let icon_response = ui
                            .add_sized(
                                self.config_ref.icon_size,
                                egui::Image::new(self.config_ref.img.unwrap().to_owned())
                                    .tint(tint_color), // Adjust color if necessary
                            )
                            .interact(Sense::click());

                        // Add some vertical spacing
                        ui.add_space(self.config_ref.icon_text_gap);

                        // Add text
                        let text_response = ui
                            .add(
                                Label::new(
                                    RichText::new(self.config_ref.text)
                                        .color(tint_color)
                                        .size(self.config_ref.text_size),
                                )
                                .selectable(false),
                            )
                            .interact(Sense::click());

                        // Update state depending upon response
                        if icon_response.clicked {
                            *self.mouse_state_ref = TexiMouseState::Clicked
                        } else if icon_response.contains_pointer() {
                            *self.mouse_state_ref = TexiMouseState::Hovering
                        } else {
                            *self.mouse_state_ref = TexiMouseState::None
                        }

                        *self.mouse_state_ref = if icon_response.clicked || text_response.clicked {
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
