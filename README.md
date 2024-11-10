## Links

* Crates.io: https://crates.io/crates/egui-widget-texicon
* docs.rs: https://docs.rs/egui-widget-texicon
* Github: https://github.com/Resonanz/egui-widget-texicon

## What is egui-widget-texicon?

egui-widget-texicon is an egui widget that combines an icon and text, often found in modern UIs.

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

### The following asumes you are using eframe_template:










In ```app.rs``` import the crate using:

```use egui_themes::{StateMachine, MOCHA};```

Using the ```TemplateApp``` struct, define a ```run_once``` boolean and a ```StateMachine``` variable to hold the current theme:

```
pub struct TemplateApp {
    run_once: bool,
    my_theme: StateMachine,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            run_once: false,
            my_theme: egui_themes::StateMachine::new(),
        }
    }
}
```

Inside ```fn update...``` set the startup theme state using the ```run_once``` boolean:

```
fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
    if self.run_once == false {
        self.run_once = true;
        self.my_theme.set_theme(ctx, &MOCHA);
    }
```
 Then in the main loop:

```
// Theme cycle button
let b = ui.add(egui::Button::new("☀🌙").sense(Sense::click()));

if b.clicked() {
    self.my_theme.rotate_theme(&ctx);
} else if b.hovered() {
    b.on_hover_text("Click for next theme...");
}
```




## Video
https://github.com/user-attachments/assets/976e0efa-d208-4eef-ac2e-24e4bb41646d
