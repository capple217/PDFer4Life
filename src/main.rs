// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
slint::include_modules!();
mod interface;
use std::sync::{Arc, Mutex};
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};


fn main() -> Result<(), slint::PlatformError> {
    /*
    Application Window
    */
    let app = App::new().unwrap();

    /*
    File Manager
    */
    let file_manager = Arc::new(interface::FileManager::new());


    //function for opening new pdf callback
    app.global::<AppService>().on_open_file({
        let app_weak = app.as_weak();
        let cloned_file_manager = file_manager.clone();
        move || {
            let app = app_weak.unwrap();
            if cloned_file_manager.add_file() {
               app.set_active_page(1);
            }
        }
    });

    app.on_close_requested({
        //let app_weak = app.as_weak();
        let cloned_file_manager = file_manager.clone();
        move || {
            //let app = app_weak.unwrap();
            let files = cloned_file_manager.getFiles();

            let json = serde_json::to_string(&files).unwrap();
            println!("the json is {}", json);
            slint::CloseRequestResponse::HideWindow
        }
        
    });

    app.run()
}