// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

mod globalCallbacks;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    globalCallbacks::defGlobalCallbacks();
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.global::<BackendTextEditor>().on_save_file(|file_name| {
        println!("ran2");
        println!("here is the text for {}", file_name);
    });

    ui.run()?;

    let sw = SplitWindow::new()?;

    sw.run();

    Ok(())
}
