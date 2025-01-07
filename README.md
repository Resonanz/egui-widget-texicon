# Texicon widget for egui

The simplest way to get the Texicon widget working is to clone the git repository, then copy the Texicon folder into the /src folder in your project.

## Using SVG icons

SVG icons can be used, however they should be created using the color 0XFFFFFF. This enables egui's tint() to be used and display the icon in the tint color.

## What is a Texicon widget?
A Texicon is an egui widget that wraps a single icon + text to form an interactive button.

By combining several Texicons, menu bars similar to Visual Studio Code's left side menu can be created.

TODO: Add an image

A Texicon is displayed using the following code where ```v``` is a vector containing the Texicon's configuration.
```
        ui.add(egui_widget_texicon::Texicon::new(v));
```
Please submit an issue on Github if you have suggestions or improvements.

## Links

* Crates.io: https://crates.io/crates/egui-widget-texicon
* docs.rs: https://docs.rs/egui-widget-texicon
* Github: https://github.com/Resonanz/egui-widget-texicon

These icons may be useful for customizing your Texicons:

* https://phosphoricons.com
* https://fonts.google.com

## Usage

**NOTE: The following assumes you are using egui's "eframe". A template is available here: https://github.com/emilk/eframe_template**

### 1. Add crate dependency
In ```Cargo.toml``` add the following dependency:
```
[dependencies]
egui-widget-texicon = 0.1.0  <--- The latest version number can be found on Crates.io.
```
Or you could use the following if developing locally:
```
[dependencies]
egui-widget-texicon = { path = "/Local_Path/egui-widget-texicon/" }
```

### 2. Add egui image_loader dependency
Texicons use images so we first need to add egui's image loader. In ```Cargo.toml``` add the following dependency:
```
# For image support
egui_extras = { version = "0.29.1", features = ["default", "all_loaders"] }
```
Your dependencies should look somethng like this:
```
[dependencies]
egui = "0.29.1"
eframe = { version = "0.29.1", default-features = false, features = ["wgpu"] }
egui-widget-texicon = "0.1.0"
egui_extras = { version = "0.29.1", features = ["default", "all_loaders"] }
```

### 3. Add the image_loader into egui
In eframe's ```main.rs``` ensure ```install_image_loaders``` is added:
```
eframe::run_native(
    "your app name",
    native_options,
    Box::new(|cc| {
        // This gives us image support:
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Box::new(your_project_name::app::TemplateApp::new(
            cc,
        )))
    }),
)
```
### 4. Import Texicon module
Import the Texicon crate into eframe's ```app.rs```.
```
use egui_widget_texicon;
```
### 5. Adding config and processing files
Texicons needs some further setup:

* First, we need a ```texicons``` folder for the following three files: ```mod.rs```, ```config.rs``` and ```process.rs```.

```
Files in src/texicons/
mod.rs
config.rs
process.rs

Contents of mod.rs:
pub mod config;
pub mod process;
```
* ```config.rs``` is used to set the icon paths and configuration for the individual Texicons using the Builder Pattern.
* ```process.rs``` contains code that receives mouse actions, processes the actions, and adjusts the Texicons accordingly. For example, a mouse action might be a hover. The code in ```process.rs``` captures the hover, performs some logic, then sets the Texicon to the hover color. (To be clear, Texicon state information is stored in the main egui code, not in the widget itself.)

### 6. Using Texicons inside ```app.rs```

```
pub struct TemplateApp<'a> {
    run_once: bool,
    // Add Texicons
    which_texi_selected: TexiSelected,
    vec_of_texis: Vec<(TexiSelected, TexiItem<'a>)>,
}

impl<'a> Default for TemplateApp<'a> {
    fn default() -> Self {
        Self {
            run_once: false,
            // Add Texicons
            which_texi_selected: TexiSelected::Gear, // Default state
            vec_of_texis: Vec::new(),
        }
    }
}
```
Fill the texicon vector once only
```
impl<'a> eframe::App for TemplateApp<'a> {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Run once
    if self.run_once == false {
      self.run_once = true;

      // Set up the vector containing the sidebar Texicons
      super::texicon::config::fill_vec(&mut self.vec_of_texis);
```
In the left side panel:
```
// Show the Texicons
ui.vertical_centered(|ui| {
    self.vec_of_texis.iter_mut().for_each(|(_, v)| {
        ui.add(egui_widget_texicon::Texicon::new(v));
    });
});
```

## Video
[Screencast from 2024-11-10 21-07-04.webm](https://github.com/user-attachments/assets/9beadb56-4573-498f-b11f-9e0dac7cdb5a)
