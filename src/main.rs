// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
slint::include_modules!();
mod file_management;


fn main() -> Result<(), slint::PlatformError> {
    /*
    Application Window
    */
    let app = App::new().unwrap();

    //functions for callback

    app.global::<AppService>().on_open_file({
        let app_weak = app.as_weak();
        
        move || {
            let app = app_weak.unwrap();
            file_management::Files::open_file();
            app.set_active_page(1);
            println!("On button clicked: id");
        }
    });

    app.run();

    Ok(())
}