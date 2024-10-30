// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
use std::error::Error;
slint::include_modules!();
mod file_management;


fn main() -> Result<(), slint::PlatformError> {
    /*
    Window: main page
    */
    let main_window = MainWindow::new()?;
    main_window.run();

    //functions for callback
    main_window.on_open_file(move || {
        println!("i came here");
        file_management::Files::open_file(); 
    });

    /*
    Window: notes splitscreen page
    */
    let notes_window = SplitWindow::new()?;
    notes_window.run();

    Ok(())
}