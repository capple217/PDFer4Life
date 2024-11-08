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

    // If we find a way to define callbacks in a different file: 
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


    // let mut text_file_path:String;
    app.global::<BackendTextEditor>().on_open_text_file(||{
        file_management::Files::open_file_txt().into()
    });

    app.global::<BackendTextEditor>().on_save_file(|file_name, text| {
        match file_management::write_to_file(file_name.as_str(), text.as_str()) {
            Ok(_) => println!("File Saved"),
            Err(e) => eprintln!("Error saving file: {}", e),
        }
    });

    app.global::<BackendTextEditor>().on_read_file(|file_name| {
        let mut text = "".to_string();
        match file_management::read_file(file_name.as_str()) {
            Ok(txt) => text = txt,
            Err(e) => eprintln!("Error loading file: {}", e),
        }
        return text.to_string().into();
    });


    app.run();

    Ok(())
}