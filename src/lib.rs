use egui::{Color32, ImageSource, Margin, Rounding, Vec2};
use egui::{Label, Response, RichText, Sense, Ui, Widget};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum TexiMouseState {
    #[default]
    None,
    Clicked,
    Hovering,
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum TexiColorState {
    #[default]
    Dim,
    On,
    Highlight,
}

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Texicon<'a> {
    pub mouse_state: &'a mut TexiMouseState,
    pub color_state: &'a mut TexiColorState,
    pub img: ImageSource<'a>,
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

// Struct for holding shared values for Texicons.
// The struct needs to be here in the widget file
// so Rust knows the struct structure whenever
// TexiSharedStates instances are passed to new().
#[derive(Default)]
pub struct TexiPortal {
    pub mouse_state: TexiMouseState,
    pub color_state: TexiColorState,
}

impl<'a> Texicon<'a> {
    pub fn new(ui: &egui::Ui, texisharedstates: &'a mut TexiPortal) -> Self {
        Texicon {
            mouse_state: &mut texisharedstates.mouse_state,
            color_state: &mut texisharedstates.color_state,
            img: egui::include_image!("../assets/pics/gear.svg"),
            text: String::from("No text defined"),
            frame_size: Vec2 { x: 60.0, y: 60.0 },
            frame_outline_width: 1.0,
            frame_outline_color: ui.visuals().weak_text_color(), //Color32::TRANSPARENT,
            frame_inner_margin: Margin::same(6.0),
            frame_outer_margin: Margin {
                left: 0.0,
                right: 0.0,
                top: 10.0,
                bottom: 10.0,
            },
            frame_rounding: Rounding::same(10.0),
            icon_size: Vec2 { x: 32.0, y: 36.0 },
            text_size: 14.0,
            icon_text_gap: 0.0,
        }
    }
    /// Set the image for the Texicon.
    #[inline]
    pub fn img(mut self, img: ImageSource<'a>) -> Self {
        self.img = img;
        self
    }

    /// Set the text for the Texicon.
    #[inline]
    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    /// Set the outer margins for the Texicon.
    #[inline]
    pub fn outer_margins(mut self, outer_margins: Margin) -> Self {
        self.frame_outer_margin = outer_margins;
        self
    }
}

impl Widget for Texicon<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Define the frame and allocate its inner UI
        let response = egui::Frame::default()
            .inner_margin(self.frame_inner_margin)
            .outer_margin(self.frame_outer_margin)
            .rounding(self.frame_rounding)
            .fill(ui.style().visuals.extreme_bg_color)
            .stroke(egui::Stroke {
                width: self.frame_outline_width,
                color: self.frame_outline_color.gamma_multiply(0.5),
            })
            .show(ui, |ui| {
                // Set the minimum size of
                // the ui (that is, the frame)
                ui.set_min_size(self.frame_size);
                ui.set_max_size(self.frame_size); // Layout the icon and text vertically with some spacing

                // Set colors for text and icon
                let tint_color = match self.color_state {
                    TexiColorState::Dim => ui.style().visuals.weak_text_color(),
                    TexiColorState::On => ui.style().visuals.text_color(),
                    TexiColorState::Highlight => ui.style().visuals.warn_fg_color,
                };

                //
                ui.allocate_ui_with_layout(
                    self.frame_size,
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        // Add icon
                        let icon_response = ui
                            .add_sized(
                                self.icon_size,
                                egui::Image::new(self.img.to_owned()).tint(tint_color), // Adjust color if necessary
                            )
                            .interact(Sense::click());

                        // Add some vertical spacing
                        ui.add_space(self.icon_text_gap);

                        // Add text
                        let text_response = ui
                            .add(
                                Label::new(
                                    RichText::new(&self.text)
                                        .color(tint_color)
                                        .size(self.text_size),
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
