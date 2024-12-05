use egui::{
    Color32, ImageSource, Label, Margin, Response, RichText, Rounding, Sense, Ui, Vec2, Widget,
};

// #[derive(Default, PartialEq)]
// pub enum TexSelectedState {
//     #[default]
//     Deselected,
//     Selected,
//     Disabled,
// }

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
pub enum TexMouseState {
    #[default]
    None,
    Clicked,
    Hovering,
}

#[derive(Default, PartialEq)]
pub enum TexColorState {
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

#[derive(Clone)]
pub struct Config<'a> {
    img: &'a ImageSource<'a>,
    text: &'a str,
    frame_size: Vec2,
    icon_size: Vec2,
    text_size: f32,
    inner_margin: Margin,
    outer_margin: Margin,
    rounding: Rounding,
    icon_text_gap: f32,
    color_light: Color32,
    color_light_hover: Color32,
    color_dark: Color32,
    color_dark_hover: Color32,
}
//
// ======================================================================
// ======================================================================
// ======================================================================
//

// Builder starts here
pub struct ConfigBuilder<'a> {
    img: &'a ImageSource<'a>,
    text: &'a str,
    frame_size: Vec2,
    icon_size: Vec2,
    text_size: f32,
    inner_margin: Margin,
    outer_margin: Margin,
    rounding: Rounding,
    icon_text_gap: f32,
    color_light: Color32,
    color_light_hover: Color32,
    color_dark: Color32,
    color_dark_hover: Color32,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new(img: &'a ImageSource<'a>) -> Self {
        ConfigBuilder {
            img,
            text: "Default text",
            frame_size: Vec2 { x: 40.0, y: 60.0 },
            icon_size: Vec2 { x: 40.0, y: 40.0 },
            text_size: 16.0,
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            rounding: Rounding::same(0.0),
            icon_text_gap: 4.0,
            color_light: Color32::PLACEHOLDER,
            color_light_hover: Color32::PLACEHOLDER,
            color_dark: Color32::PLACEHOLDER,
            color_dark_hover: Color32::PLACEHOLDER,
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

    pub fn color_light(mut self, color_light: Color32) -> Self {
        self.color_light = color_light;
        self
    }

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
            color_light: self.color_light,
            color_light_hover: self.color_light_hover,
            color_dark: self.color_dark,
            color_dark_hover: self.color_dark_hover,
        }
    }
}
//
// ======================================================================
// ======================================================================
// ======================================================================
//

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
pub struct SidebarTexicon<'a> {
    // pub texi_state: TexSelectedState,
    pub texi_mouse: TexMouseState,
    pub texi_color: TexColorState,
    pub config: Config<'a>,
}

impl<'a> SidebarTexicon<'a> {
    pub fn new(config: Config<'a>) -> Self {
        Self {
            // texi_state: TexSelectedState::Deselected,
            texi_mouse: TexMouseState::None,
            texi_color: TexColorState::Dim,
            config,
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
    // on_off_state: &'a mut TexSelectedState,
    mouse_state: &'a mut TexMouseState,
    color_state: &'a mut TexColorState,
    config: &'a Config<'a>,
}

impl<'a> Texicon<'a> {
    pub fn new(sidebar_texicon: &'a mut SidebarTexicon) -> Self {
        Self {
            // on_off_state: &mut sidebar_texicon.texi_state,
            mouse_state: &mut sidebar_texicon.texi_mouse,
            color_state: &mut sidebar_texicon.texi_color,
            config: &mut sidebar_texicon.config,
        }
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
            .show(ui, |ui| {
                // Set the minimum size of
                // the ui (that is, the frame)
                ui.set_min_size(self.config.frame_size);
                ui.set_max_size(self.config.frame_size); // Layout the icon and text vertically with some spacing

                let tint_color = match *self.color_state {
                    TexColorState::Dim => ui.style().visuals.weak_text_color(),
                    TexColorState::On => ui.style().visuals.strong_text_color(),
                    TexColorState::Highlight => ui.style().visuals.error_fg_color,
                };

                // A TexColorState???
                ui.allocate_ui_with_layout(
                    self.config.frame_size,
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        // Add icon
                        let icon_response = ui
                            .add_sized(
                                self.config.icon_size,
                                egui::Image::new(self.config.img.to_owned()).tint(tint_color), // Adjust color if necessary
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
                            *self.mouse_state = TexMouseState::Clicked
                        } else if icon_response.contains_pointer() {
                            *self.mouse_state = TexMouseState::Hovering
                        } else {
                            *self.mouse_state = TexMouseState::None
                        }

                        *self.mouse_state = if icon_response.clicked || text_response.clicked {
                            TexMouseState::Clicked
                        } else if icon_response.contains_pointer()
                            || text_response.contains_pointer()
                        {
                            TexMouseState::Hovering
                        } else {
                            TexMouseState::None
                        }
                    },
                );
            })
            .response;

        response
    }
}
