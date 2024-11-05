// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
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
        println!("On button clicked: id");
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