#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

extern crate libui;
use libui::controls::*;
use libui::layout;
use libui::prelude::*;

fn main() {
    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(
        &ui,
        "VSCode Portable Quick Launcher",
        800,
        600,
        WindowType::NoMenubar,
    );

    layout! {
        &ui,
        let layout = VerticalBox() {
            Stretchy: let container = VerticalBox(padded: true) {
                Compact: let choices = Combobox(selected: 0) {
                    "one",
                    "two",
                    "three"
                }
                Compact: let hbox = HorizontalBox(padded: true) {
                    Stretchy: let _sp = Spacer()
                    Compact: let button = Button("Open VSCode Portable")
                    Stretchy: let _sp = Spacer()
                }
            }
        }
    }

    win.set_child(layout);
    win.show();
    ui.main();
}
