// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
use std::error::Error;

mod globalCallbacks;

slint::include_modules!();
mod file_management;


fn main() -> Result<(), slint::PlatformError> {
    /*
    Window: main page
    */
    let app = App::new().unwrap();

    /*
    Window: notes splitscreen page
    */
    //let notes_window = SplitWindow::new()?;
    //notes_window.run();

    //functions for callback

    app.global::<AppService>().on_open_file(|| {
        let app_weak = app.as_weak();
        
        move || {
            let app = app_weak.unwrap();
            
        }
    });

    app.global::<BackendTextEditor>().on_save_file(|file_name, text| {
        println!("ran2");
        println!("here is the text for {}: {}", file_name, text);
    });

    // on_open_file({
    //     let app_weak = app.as_weak();
        
    //     move || {

    //     file_management::Files::open_file();
        
    //     app_weak.unwrap().active_page = 1;

    //     }
    // });

    app.run();

    Ok(())
}