#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

extern crate libui;
use libui::controls::*;
use libui::prelude::*;

fn main() {
    let ui = UI::init()
        .expect("Couldn't initialize UI library");
    
    let mut win = Window::new(&ui, "Example", 800, 600, 
        WindowType::NoMenubar);
    let layout = VerticalBox::new();

    // add controls to your layout here

    win.set_child(layout);
    win.show();
    ui.main();
}

