// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
use std::error::Error;

slint::include_modules!();
mod file_management;
mod globalCallbacks;

fn main() -> Result<(), slint::PlatformError> {
    /*
    Application Window
    */
    let app = App::new().unwrap();

    //functions for callback
    let app_weak = app.as_weak();
    // globalCallbacks::defGlobalCallbacks(app_weak.unwrap());

    app.global::<AppService>().on_open_file({
        let app_weak = app.as_weak();
        
        move || {
            let app = app_weak.unwrap();
            file_management::Files::open_file();
            app.set_active_page(1);
            println!("On button clicked: id");
        }
    });

    app.global::<BackendTextEditor>().on_save_file(|file_name, text| {
        println!("ran2");
        println!("here is the text for {}: {}", file_name, text);
    });

    app.run();

    Ok(())
}