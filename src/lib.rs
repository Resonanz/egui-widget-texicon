use egui::{
    Color32, Image, ImageSource, Pos2, Rect, Response, Rounding, Sense, Stroke, TextStyle,
    TextWrapMode, Ui, Vec2, Widget, WidgetText,
};

#[derive(Clone)]
pub struct Config<'a> {
    pub icon_width: f32,
    pub icon_height: f32,
    pub text_width: f32,
    pub text_height: f32,
    pub icon_text_gap: f32,
    pub color_light: Color32,
    pub color_light_hover: Color32,
    pub color_dark: Color32,
    pub color_dark_hover: Color32,
    pub text: &'a str,
    pub img: ImageSource<'a>,
    // font_family: FontId,
}

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Texicon<'a> {
    config: Config<'a>,
}

impl<'a> Texicon<'a> {
    pub fn new(config: Config<'a>) -> Self {
        Self { config }
    }
}

impl Widget for Texicon<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Configure depending upon whether
        // text + icon or just icon are present.
        let (galley, desired_size) = if self.config.text.is_empty() {
            (
                None,
                Vec2::new(self.config.icon_width, self.config.icon_height),
            )
        } else {
            // Convert incoming text to WidgetText
            let text: WidgetText = self.config.text.into();

            // Add text into galley with some layout formatting
            let text = text.into_galley(
                ui,
                Some(TextWrapMode::Wrap),
                self.config.text_width,
                TextStyle::Button,
            );

            let galley_width = text.rect.max.x;
            let galley_height = text.rect.max.y;

            // Assumes text and icon are related vertically
            let desired_size = Vec2::new(
                f32::max(self.config.icon_width, galley_width),
                self.config.icon_height + self.config.icon_text_gap + galley_height,
            );

            (Some(text), desired_size)
        };

        // allocate_exact_size: "Returns a Rect with exactly what you asked for."
        // rect: "A rectangular region of space."
        // response: "A Response lets you know whether or not
        //   a widget is being hovered, clicked or dragged. It
        //   also lets you easily show a tooltip on hover."
        // Get absolute coordinates of rectangle
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        // 4. Paint!
        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

            // Determine if hover and set flag
            // for later hover color settings
            let mut hovered_state: bool = false;
            if response.hovered() {
                hovered_state = true;
            }

            // Calculate positions of icon corners (top-left and bottom-right)
            // relative to the rect top-left (0,0) corner.
            let icon_min_x = (rect.width() - self.config.icon_width) / 2.;
            let icon_min_y = 0.;
            let icon_max_x = icon_min_x + self.config.icon_width;
            let icon_max_y = 0. + self.config.icon_height;

            // Add the absolute coordinates of the icon to the
            // absolute window coordinates of rect.
            let rect_pos_icon = Rect {
                min: Pos2 {
                    x: rect.min.x + icon_min_x,
                    y: rect.min.y + icon_min_y,
                },
                max: Pos2 {
                    x: rect.min.x + icon_max_x,
                    y: rect.min.y + icon_max_y,
                },
            };

            let i: Image<'_> = self.config.img.into();

            let tlr = i.load_for_size(ui.ctx(), desired_size); // This stands for "texture load result"
            match tlr {
                // Check our texture is ready and waiting
                Ok(egui::load::TexturePoll::Ready { texture }) => {
                    //println!("TexturePoll::Ready !!! texture.id = {:?}", texture.id);
                    // paint_texture_at(ui.painter(), rect, options, texture);
                    ui.painter().add(egui::epaint::RectShape {
                        rect: rect_pos_icon,
                        rounding: Rounding::ZERO,
                        fill: if hovered_state {
                            visuals.fg_stroke.color
                        } else {
                            visuals.bg_stroke.color
                        },
                        stroke: Stroke::NONE,
                        blur_width: 0.0,
                        fill_texture_id: texture.id,
                        uv: Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
                    });

                    if let Some(galley) = galley {
                        let text_pos = Pos2::new(
                            rect.min.x + 0.,
                            rect.min.y + self.config.icon_height + self.config.icon_text_gap,
                        );
                        ui.painter().galley(text_pos, galley, visuals.text_color());
                    }
                }
                // Ouch -- the texture failed to load. This should never happen !!!
                _ => {
                    println!("Texture Load Result (tlr) error");
                }
            }
        }
        response
    }
}
