// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
use std::error::Error;
slint::include_modules!();


fn main() -> Result<(), slint::PlatformError> {
    /*
    Window: main page
    */
    let main_window = MainWindow::new()?;
    main_window.run();

    //functions for callback
    main_window.on_open_file(move || {
        Files::open_file(); 
    });

    /*
    Window: notes splitscreen page
    */
    let notes_window = SplitWindow::new()?;
    notes_window.run();
}

use native_dialog::FileDialog;

#[derive(Default)]
pub struct Files;

impl Files {
    pub fn open_file() {
        if let Some(path) = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .show_open_single_file()
        .unwrap() {
            println!("Selected file: {:?}", path);
        }
    }
}

// //example
// slint::include_modules!();

// fn main() -> Result<(), Box<dyn Error>> {
//     let ui = AppWindow::new()?;

//     ui.on_request_increase_value({
//         let ui_handle = ui.as_weak();
//         move || {
//             let ui = ui_handle.unwrap();
//             ui.set_counter(ui.get_counter() + 1);
//         }
//     });

//     ui.run()?;

//     Ok(())
// }