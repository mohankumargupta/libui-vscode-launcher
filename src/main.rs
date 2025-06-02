#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

mod vscode;

extern crate libui;
use libui::controls::*;
use libui::layout;
use libui::prelude::*;


fn get_vscode_folders() -> Result<Vec<String>, std::io::Error> {
    vscode::get_vscode_portable_folder_names()
}

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let download_directory = vscode::downloads_dir().unwrap_or("".into());
    let folders = get_vscode_folders()?;
    
    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(
        &ui,
        "VSCode Portable Quick Launcher",
        800,
        600,
        WindowType::NoMenubar,
    );

    //Static layout
    layout! {
        &ui,
        let layout = VerticalBox() {
            Stretchy: let container = VerticalBox(padded: true) {
                Compact: let choices = Combobox() {
                    
                }
                Compact: let hbox = HorizontalBox(padded: true) {
                    Stretchy: let _sp = Spacer()
                    Compact: let button = Button("Open VSCode Portable")
                    Stretchy: let _sp = Spacer()
                }
                Compact: let hbox = HorizontalBox(padded: true) {
                    Stretchy: let _sp = Spacer()
                    Compact: let debugging = Label("Debugging...")
                    Stretchy: let _sp = Spacer()
                }                
            }
        }
    }

    //Dynamic loading initial data

    for folder in &folders {
        choices.append(&folder);
    }
    choices.set_selected(0);
    button.on_clicked(move |_|{
        let index = choices.selected();
        if let &Some( folder) = &folders.get(index as usize) {
            let mut fullpath = download_directory.clone();
            fullpath.push(&folder);
            debugging.set_text(&fullpath.to_string_lossy());
        }
        // let dir = folders.index(index);
        // debugging.set_text(dir);
    });

    win.set_child(layout);
    win.show();
    ui.main();
    Ok(())
}
