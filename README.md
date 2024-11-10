## Links

* Crates.io: https://crates.io/crates/egui-widget-texicon
* docs.rs: https://docs.rs/egui-widget-texicon
* Github: https://github.com/Resonanz/egui-widget-texicon

### Links to icons

* https://phosphoricons.com
* https://fonts.google.com

## What is egui-widget-texicon?

egui-widget-texicon is an egui widget that combines an icon and text. Such an arrangement is often found in modern UIs.

Please submit an issue on Github if you have suggestions or improvements.

## Usage

In ```Cargo.toml``` add the following dependency:

```
[dependencies]
egui-widget-texicon = 0.1.0  <--- The latest version number can be found on Crates.io.
```

Or you could use the following if developing locally:
```
[dependencies]
egui-widget-texicon = { path = "/Github/egui-widget-texicon/" }
```

Import the crate using

```
use egui_widget_texicon::{Config, Texicon};
```

To add images, define a constant that points to each image file.

As illustrated below, these can be PNG or SVG or other image formats supported by egui's image_loader (https://docs.rs/egui/latest/egui/macro.include_image.html).

Note that loading images using the ```include_image!``` macro bakes the raw image bytes into the binary file.

```
const IMG_CAMERA: egui::ImageSource<'_> = egui::include_image!("../assets/pics/google-camera.png");
const IMG_SCATTER: egui::ImageSource<'_> = egui::include_image!("../assets/pics/chart-scatter.png");
const IMG_PROCESS: egui::ImageSource<'_> = egui::include_image!("../assets/pics/google-grain.png");
const IMG_IOS192: egui::ImageSource<'_> = egui::include_image!("../assets/pics/gear.svg");
```

Create a left side SidePanel, define the button details including button ```text```, and add the ```Texicon``` to the SidePanel.

```
SidePanel::left("Left panel").show(ctx, |ui| {

    let image_name = Config {
        icon_width: 32.,
        icon_height: 32.,
        text_width: 80.,
        text_height: 80.,
        icon_text_gap: 4.,
        color_light: Color32::DARK_GRAY,
        color_light_hover: Color32::BLACK,
        color_dark: Color32::GRAY,
        color_dark_hover: Color32::WHITE,
        text: "Settings",
        img: IMG_IOS192,
    };

    ui.vertical_centered(|ui| {
        ui.add_space(10.);
        if ui.add(Texicon::new(image_name)).clicked() {  // Texicon 1
            println!("Clicked btn 1");
        };
        ui.add_space(10.);
        if ui.add(Texicon::new(image_name)).clicked() {  // Texicon 2
            println!("Clicked btn 2");
        };
        ui.add_space(10.);
        if ui.add(Texicon::new(image_name)).clicked() {  // Texicon 3
            println!("Clicked btn 3");
        };
        ui.add_space(10.);
        warn_if_debug_build(ui);
    });
});
```

## Video
[Screencast from 2024-11-10 21-07-04.webm](https://github.com/user-attachments/assets/9beadb56-4573-498f-b11f-9e0dac7cdb5a)

