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

    // main_window.on_open_file({
    //     let main_window_weak = main_window.as_weak();
    //     let notes_window_weak = notes_window.as_weak();
        
    //     move || {

    //     file_management::Files::open_file();
    //     main_window_weak.unwrap().hide(); 
    //     notes_window_weak.unwrap().run();

    //     }
    // });

    app.run();

    Ok(())
}