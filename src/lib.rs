use egui::{Color32, ImageSource, Label, Margin, Response, RichText, Sense, Ui, Vec2, Widget};

// mod defs_and_consts;

#[derive(Default, PartialEq)]
pub enum TexSelectedState {
    #[default]
    Deselected,
    Selected,
    Disabled,
}

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
    outer_margin_left: f32,
    outer_margin_right: f32,
    outer_margin_top: f32,
    outer_margin_bottom: f32,
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
    outer_margin_left: f32,
    outer_margin_right: f32,
    outer_margin_top: f32,
    outer_margin_bottom: f32,
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
            frame_size: Vec2 { x: 100.0, y: 60.0 },
            icon_size: Vec2 { x: 40.0, y: 40.0 },
            text_size: 16.0,
            outer_margin_left: 0.0,
            outer_margin_right: 0.0,
            outer_margin_top: 0.0,
            outer_margin_bottom: 0.0,
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

    pub fn outer_margin_left(mut self, outer_margin_left: f32) -> Self {
        self.outer_margin_left = outer_margin_left;
        self
    }

    pub fn outer_margin_right(mut self, outer_margin_right: f32) -> Self {
        self.outer_margin_right = outer_margin_right;
        self
    }

    pub fn outer_margin_top(mut self, outer_margin_top: f32) -> Self {
        self.outer_margin_top = outer_margin_top;
        self
    }

    pub fn outer_margin_bottom(mut self, outer_margin_bottom: f32) -> Self {
        self.outer_margin_bottom = outer_margin_bottom;
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
            outer_margin_left: self.outer_margin_left,
            outer_margin_right: self.outer_margin_right,
            outer_margin_top: self.outer_margin_top,
            outer_margin_bottom: self.outer_margin_bottom,
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
    pub texi_state: TexSelectedState,
    pub texi_mouse: TexMouseState,
    pub texi_color: TexColorState,
    pub config: Config<'a>,
}

impl<'a> SidebarTexicon<'a> {
    pub fn new(config: Config<'a>) -> Self {
        Self {
            texi_state: TexSelectedState::Deselected,
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
    on_off_state: &'a mut TexSelectedState,
    mouse_state: &'a mut TexMouseState,
    color_state: &'a mut TexColorState,
    config: &'a Config<'a>,
}

impl<'a> Texicon<'a> {
    pub fn new(sidebar_texicon: &'a mut SidebarTexicon) -> Self {
        Self {
            on_off_state: &mut sidebar_texicon.texi_state,
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
            .inner_margin(Margin {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            })
            .outer_margin(Margin {
                left: self.config.outer_margin_left,
                right: self.config.outer_margin_right,
                top: self.config.outer_margin_top,
                bottom: self.config.outer_margin_bottom,
            })
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

                // ATexColorState
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
