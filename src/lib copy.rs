use egui::{Color32, ImageSource, Label, Margin, Response, Sense, Ui, Vec2, Widget};

mod defs_and_consts;
use defs_and_consts::{
    DEFAULT_TEXT, FRAME_SIZE, ICON_SIZE, ICON_TEXT_SPACE, TEXT_SIZE, WIDGET_SPACE_ABOVE,
    WIDGET_SPACE_BELOW,
};

// /// Configuration for the
// /// widget appearance
// //#[derive(Clone)]
// struct WidgetFrame {
//     // Frame config
//     size_image: Vec2,
//     size_frame: Vec2,
//     // stroke: Stroke,
//     // stroke_hover: Stroke,
//     // fill: Color32,
//     // fill_hover: Color32,
// }

// impl Default for WidgetFrame {
//     fn default() -> Self {
//         Self {
//             // Frame
//             size_image: Vec2::new(60.0, 60.0),
//             size_frame: Vec2::new(100.0, 100.0),
//             // stroke: Stroke::new(1.0, Color32::from_gray(100)),
//             // stroke_hover: Stroke::new(1.0, Color32::WHITE),
//             // fill: Color32::BLACK,
//             // fill_hover: Color32::from_black_alpha(123),
//         }
//     }
// }

//
// ======================================================================
// ======================================================================
// ======================================================================
//

#[derive(Default, PartialEq)]
pub enum OnOffState {
    #[default]
    Off,
    On,
}

#[derive(Default, PartialEq)]
pub enum MouseState {
    #[default]
    Off,
    Clicked,
    Hovering,
}

#[derive(Default, PartialEq)]
pub enum ColorState {
    #[default]
    Off,
    On,
    OnBright,
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
    text_size: Vec2,
    widget_space_above: f32,
    widget_space_below: f32,
    icon_text_space: f32,
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
    text_size: Vec2,
    widget_space_above: f32,
    widget_space_below: f32,
    icon_text_space: f32,
    color_light: Color32,
    color_light_hover: Color32,
    color_dark: Color32,
    color_dark_hover: Color32,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new(img: &'a ImageSource<'a>) -> Self {
        ConfigBuilder {
            img,
            text: DEFAULT_TEXT,
            frame_size: FRAME_SIZE,
            icon_size: ICON_SIZE,
            text_size: TEXT_SIZE,
            widget_space_above: WIDGET_SPACE_ABOVE,
            widget_space_below: WIDGET_SPACE_BELOW,
            icon_text_space: ICON_TEXT_SPACE,
            color_light: Color32::DARK_GRAY,
            color_light_hover: Color32::BLACK,
            color_dark: Color32::GRAY,
            color_dark_hover: Color32::WHITE,
        }
    }

    pub fn text(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }

    pub fn icon_size(mut self, icon_size: Vec2) -> Self {
        self.icon_size = icon_size;
        self
    }

    pub fn text_size(mut self, text_size: Vec2) -> Self {
        self.text_size = self.text_size;
        self
    }

    pub fn build(self) -> Config<'a> {
        Config {
            img: self.img,
            text: self.text,
            frame_size: self.frame_size,
            icon_size: self.icon_size,
            text_size: self.text_size,
            widget_space_above: self.widget_space_above,
            widget_space_below: self.widget_space_below,
            icon_text_space: self.icon_text_space,
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
    pub texicon_camera_on_off: OnOffState,
    pub texicon_camera_mouse: MouseState,
    pub texicon_camera_color: ColorState,
    pub config: Config<'a>,
}

impl<'a> SidebarTexicon<'a> {
    pub fn new(image: &'a ImageSource<'a>) -> Self {
        Self {
            texicon_camera_on_off: OnOffState::Off,
            texicon_camera_mouse: MouseState::Off,
            texicon_camera_color: ColorState::Off,
            config: ConfigBuilder::new(&image).build(),
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
    on_off_state: &'a mut OnOffState,
    mouse_state: &'a mut MouseState,
    color_state: &'a mut ColorState,
    config: &'a Config<'a>,
}

impl<'a> Texicon<'a> {
    pub fn new(sidebar_texicon: &'a mut SidebarTexicon) -> Self {
        Self {
            on_off_state: &mut sidebar_texicon.texicon_camera_on_off,
            mouse_state: &mut sidebar_texicon.texicon_camera_mouse,
            color_state: &mut sidebar_texicon.texicon_camera_color,
            config: &mut sidebar_texicon.config,
        }
    }
}

// on_off_state: &'a mut OnOffState,
// mouse_state: &'a mut MouseState,
// color_state: &'a mut ColorState,
// config: &'a Config,

// on_off_state,
// mouse_state,
// color_state,
// config,

// impl<'a> Widget for Texicon<'a> {
//     fn ui(self, ui: &mut Ui) -> Response {
//         // Instantiate the required
//         // widget building blocks
//         // let widget_frame = WidgetFrame::default();
//         //
//         //
//         // Define the frame
//         //
//         let response = egui::Frame::default()
//             .inner_margin(0.0)
//             .outer_margin(0.0)
//             .show(ui, |ui| {
//                 // let visuals = ui.style().visuals;

//                 // Set the minimum size of
//                 // the ui (that is, the frame)
//                 // ui.set_min_size(widget_frame.size);
//                 // ui.set_max_size(widget_frame.size);
//                 let origin = ui.min_rect().min;

//                 // const IMG2: egui::ImageSource<'_> =
//                 //     egui::include_image!("../assets/pics/testtube_black.svg");
//                 // let i = self.config.img;

//                 // Draw image
//                 let both_responses = ui.allocate_ui_with_layout(
//                     Vec2 {
//                         x: self.config.icon_width,
//                         y: self.config.icon_height,
//                     }, // Desired size
//                     egui::Layout::top_down(egui::Align::Center), // Layout configuration
//                     |ui| {
//                         ui.add_space(self.config.widget_gap_above);
//                         let icon_response = ui
//                             .add_sized(
//                                 [self.config.icon_width, self.config.icon_height],
//                                 egui::Image::new(self.config.img.to_owned())
//                                     .tint(ui.style().visuals.warn_fg_color),
//                             )
//                             .interact(Sense::click());

//                         ui.add_space(self.config.widget_gap_above);
//                         let text_response = ui
//                             .allocate_rect(
//                                 Rect {
//                                     min: pos2(origin.x + 0.0, origin.y + 20.0),
//                                     max: pos2(origin.x + 40.0, origin.y + 40.0),
//                                 },
//                                 Sense::click(),
//                             )
//                             .interact(Sense::click());

//                         // UI contents go here
//                         ui.label("Left-aligned content");
//                         ui.button("Some button");
//                         (icon_response, text_response)
//                     },
//                 );
//                 // Combine the responses
//                 let mut response = both_responses.response;
//                 // Update the response's clicked state based on either rectangle being clicked
//                 let (icon_response, text_response) = both_responses.inner;
//                 if icon_response.clicked() || text_response.clicked() {
//                     response.mark_changed();
//                 }

//                 // Determine if hover and set flag
//                 // for later hover color settings

//                 // Update mouse state based on the interaction with the widget
//                 *self.mouse_state = if response.clicked() {
//                     MouseState::Clicked
//                 } else if response.contains_pointer() {
//                     MouseState::Hovering
//                 } else {
//                     MouseState::Off
//                 };

//                 response
//             });
//         response.inner
//     }
// }

impl<'a> Widget for Texicon<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Define the frame and allocate its inner UI
        let response = egui::Frame::default()
            .inner_margin(Margin {
                left: 0.0,
                right: 0.0,
                top: self.config.widget_space_above,
                bottom: 0.0,
            })
            .outer_margin(0.0)
            .show(ui, |ui| {
                // Set the minimum size of
                // the ui (that is, the frame)
                ui.set_min_size(self.config.frame_size);
                ui.set_max_size(self.config.frame_size); // Layout the icon and text vertically with some spacing

                // Add frame
                ui.allocate_ui_with_layout(
                    self.config.frame_size,
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        // Add icon
                        let icon_response = ui
                            .add_sized(
                                self.config.icon_size,
                                egui::Image::new(self.config.img.to_owned())
                                    .tint(ui.style().visuals.warn_fg_color), // Adjust color if necessary
                            )
                            .interact(Sense::click());

                        // Add some vertical spacing
                        ui.add_space(self.config.icon_text_space);

                        // Add text
                        let text_response = ui
                            .add(Label::new("Configuration").selectable(false))
                            .interact(Sense::click());

                        // Update state depending upon response
                        if icon_response.clicked {
                            *self.mouse_state = MouseState::Clicked
                        } else if icon_response.contains_pointer() {
                            *self.mouse_state = MouseState::Hovering
                        } else {
                            *self.mouse_state = MouseState::Off
                        }

                        *self.mouse_state = if icon_response.clicked || text_response.clicked {
                            MouseState::Clicked
                        } else if icon_response.contains_pointer()
                            || text_response.contains_pointer()
                        {
                            MouseState::Hovering
                        } else {
                            MouseState::Off
                        }
                    },
                );
            })
            .response;

        response
    }
}
