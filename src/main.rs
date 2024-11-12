// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
slint::include_modules!();
mod interface;
use std::sync::{Arc, Mutex};


fn main() -> Result<(), slint::PlatformError> {
    /*
    Application Window
    */
    let app = App::new().unwrap();

    /*
    File Manager
    */
    let file_manager = Arc::new(Mutex::new(interface::FileManager::new()));


    //function for opening new pdf callback
    app.global::<AppService>().on_open_file({
        let app_weak = app.as_weak();
        let cloned_file_manager = file_manager.clone();
        move || {
            let app = app_weak.unwrap();
            if cloned_file_manager.unwrap().lock().unwrap().add_file() {
`               app.set_active_page(1);
            }
        }
    });

    app.run()
}