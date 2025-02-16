//! # Creating Texicon widgets
//!
//! To use Texicons we must do the following:
//!
//! 1. Define each Texicon. At a minimum this means specifying an image and text.
//!
//! 2. Define two portal variables for sharing mouse and color states between widget code and the main code base for each Texicon.
//!
//! 3. Define an enum for each Texicon to keep tabs on which one is currently selected.
//!
//! 4. Define a vector to contain the above information about each individual Texicon.
//!
//! 5. Draw the Texicons by iterating over the vector and adding each item as an egui widget.
//!
//! 6. Process the mouse states (None, Clicked, Hovering) for all Texicons to
//!    set the selected Texicons and the color changes due to mouse presence.
//!
//! This is not as difficult as it sounds. A working example can be found in the Github repository.
//!
//! Incomplete details follow.
//!
//! A vector in ```app.rs``` (assuming "eframe Template" is being used) is defined containing
//! a tuple-triplet as shown below. ```TexiType``` and ```TexiState``` are enums to hold Texicon states.
//! ```TexiPortal``` is a struct containing two variables for sharing state between the widget and main code base.
//! Each Texicon has a tuple-triplet entry in the vector.
//! ```
//! pub struct TemplateApp {
//!     which_texi_selected: TexiType,
//!     texis_vec: Vec<(TexiType, TexiPortal, TexiState)>,
//! }
//!```
//! The vector is iterated over and each Texicon's ```TexiPortal``` struct is passed into the widget code to create individual shared references.
//! ```
//! ui.add(
//!     // Pass the TexiPortal struct into the widget.
//!     Texicon::new(&ui, &mut texis_vec[0].1)
//!         .img(egui::include_image!("../../assets/pics/send.svg"))
//!         .text(String::from("Send"))
//!         // Add a bit extra at the
//!         // top for nice aesthetics
//!         .frame_outer_margin(Margin {
//!             left: 0.0,
//!             right: 0.0,
//!             top: 20.0,
//!             bottom: 10.0,
//!         }),
//! );
//!
//! // Add another Texicon...
//! ui.add(
//!    Texicon::new(&ui, &mut texis_vec[1].1)
//!        .img(egui::include_image!("../../assets/pics/recv.svg"))
//!        .text(String::from("Receive")),
//! );
//! ```
//! The shared references are then "processed" (the biz logic) inside the main code
//! base -- the mouse_states are the inputs and the color_states are the outputs.
//!
//! If a series of Texicons are displayed in the left-side panel, the selected
//! Texicon can be matched as shown below and the appropriate CentraPanel displayed.
//!
//! ```
//! CentralPanel::default().show(ctx, |ui| {
//!     do_central_panel(
//!         match self.which_texi_selected {
//!             TexiType::Send => { /* Display Send layout */ },
//!             TexiType::Recv => { /* Display Receive layout */ },
//!             TexiType::Sett => { /* Display Settings layout */ },
//!         },
//!     );
//! });
//! ```

use egui::{Color32, CornerRadius, ImageSource, Margin, Vec2};
use egui::{Label, Response, RichText, Sense, Ui, Widget};

/// Texicon mouse interaction states
#[derive(Default)]
pub enum TexiMouseState {
    #[default]
    None,
    Clicked,
    Hovering,
}

/// Texicon color states (colors differ with different themes)
#[derive(Default)]
pub enum TexiColorState {
    #[default]
    Dim,
    On,
    Highlight,
}

/// Texicon information store
///
/// ```mouse_state``` and ```color_state``` are references, used
/// as state-sharing portals between widget and main code bases.
///
/// ```img``` is the Texicon image (e.g. PNG, SVG). Images are unknown
/// to the widget so must be initialized within the main code base.
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
    pub frame_rounding: CornerRadius,
    pub icon_size: Vec2,
    pub text_size: f32,
    pub icon_text_gap: f32,
}

/// Struct for holding shared values for Texicons.
///
/// The struct has to be defined in the widget file so
/// Rust can know the structure of the struct whenever
/// TexiSharedStates instances are passed into new().
#[derive(Default)]
pub struct TexiPortal {
    pub mouse_state: TexiMouseState,
    pub color_state: TexiColorState,
}

/// Default values for the Texicon struct and
/// a Builder Pattern implementation for customization.
impl<'a> Texicon<'a> {
    pub fn new(ui: &egui::Ui, texisharedstates: &'a mut TexiPortal) -> Self {
        Texicon {
            mouse_state: &mut texisharedstates.mouse_state,
            color_state: &mut texisharedstates.color_state,
            img: egui::include_image!("../assets/pics/gear.svg"),
            text: String::from("No text defined"),
            frame_size: Vec2 { x: 60.0, y: 60.0 },
            frame_outline_width: 1.0,
            frame_outline_color: ui.visuals().weak_text_color(),
            frame_inner_margin: Margin::same(6),
            frame_outer_margin: Margin {
                left: 0,
                right: 0,
                top: 10,
                bottom: 10,
            },
            frame_rounding: egui::CornerRadius::same(10),
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

    /// Set the frame_size for the Texicon.
    #[inline]
    pub fn frame_size(mut self, frame_size: Vec2) -> Self {
        self.frame_size = frame_size;
        self
    }

    /// Set the frame_outline_width for the Texicon.
    #[inline]
    pub fn frame_outline_width(mut self, frame_outline_width: f32) -> Self {
        self.frame_outline_width = frame_outline_width;
        self
    }

    /// Set the frame_outline_color for the Texicon.
    #[inline]
    pub fn frame_outline_color(mut self, frame_outline_color: Color32) -> Self {
        self.frame_outline_color = frame_outline_color;
        self
    }

    /// Set the frame_inner_margin for the Texicon.
    #[inline]
    pub fn frame_inner_margin(mut self, frame_inner_margin: Margin) -> Self {
        self.frame_inner_margin = frame_inner_margin;
        self
    }

    /// Set the frame_outer_margin for the Texicon.
    #[inline]
    pub fn frame_outer_margin(mut self, frame_outer_margin: Margin) -> Self {
        self.frame_outer_margin = frame_outer_margin;
        self
    }

    /// Set the frame_rounding for the Texicon.
    #[inline]
    pub fn frame_rounding(mut self, frame_rounding: CornerRadius) -> Self {
        self.frame_rounding = frame_rounding;
        self
    }

    /// Set the icon_size for the Texicon.
    #[inline]
    pub fn icon_size(mut self, icon_size: Vec2) -> Self {
        self.icon_size = icon_size;
        self
    }

    /// Set the text_size for the Texicon.
    #[inline]
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    /// Set the icon_text_gap for the Texicon.
    #[inline]
    pub fn icon_text_gap(mut self, icon_text_gap: f32) -> Self {
        self.icon_text_gap = icon_text_gap;
        self
    }
}

/// Widget trait to enable the Texicon widget to be displayed
/// using the standard egui ```ui.add(Texicon::new(...))```
///
impl Widget for Texicon<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Define the frame and allocate its inner UI
        let response = egui::Frame::default()
            .inner_margin(self.frame_inner_margin)
            .outer_margin(self.frame_outer_margin)
            .corner_radius(self.frame_rounding)
            .fill(match self.color_state {
                TexiColorState::Highlight => ui.style().visuals.extreme_bg_color,
                _ => ui.style().visuals.panel_fill,
            })
            .stroke(egui::Stroke {
                width: self.frame_outline_width,
                color: self.frame_outline_color,
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
                        *self.mouse_state = if icon_response.clicked() || text_response.clicked() {
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
