// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
use std::error::Error;
use pdfium_render::prelude::*;

slint::include_modules!();
mod interface;
mod file_management;
mod pdf_renderer;
use std::sync::{Arc, Mutex};
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};


fn main() -> Result<()> { //ideally result should also have: Result<(), slint::PlatformError>
    /*
    Application Window
    */
    let app = App::new().unwrap();

    /*
    File Manager
    */
    let file_manager = Arc::new(Mutex::new(interface::FileManager::new()));


    /*  CALLBACK:
        Prompts user selects PDF -> switch to text-editor pdf-render splitscreen page
    */
    app.global::<AppService>().on_open_file({
        let app_weak = app.as_weak();
        let cloned_file_manager = file_manager.clone();
        move || {
            let app = app_weak.unwrap();
            if cloned_file_manager.lock().unwrap().add_file() {
               app.set_active_page(1);
            }
        }
    });

    // app.on_close_requested({
    //     //let app_weak = app.as_weak();
    //     let cloned_file_manager = file_manager.clone();
    //     move || {
    //         //let app = app_weak.unwrap();
    //         let files = cloned_file_manager.lock().unwrap().getFiles();

    //         let json = serde_json::to_string(&files).unwrap();
    //         println!("the json is {}", json);
    //         slint::CloseRequestResponse::HideWindow
    //     }
        
    // });


    //BACKEND PDF
    // pure callback navigate_previous();
    // pure callback navigate_next();
    app.global::<BackendPDF>().on_navigate_previous(||{
        
    });


    
    /*  CALLBACK:
        Prompt user to select txt file
        Returns path to txt file as String
    */
    app.global::<BackendTextEditor>().on_open_text_file(||{
        file_management::Files::open_file_txt().into()
    });

    /*  CALLBACK:
            file_name: String = file path
            text: String = text to save
        Saves text to specified file path
        Returns void
    */
    app.global::<BackendTextEditor>().on_save_file(|file_name, text| {
        match file_management::write_to_file(file_name.as_str(), text.as_str()) {
            Ok(_) => println!("File Saved"),
            Err(e) => eprintln!("Error saving file: {}", e),
        }
    });

    /*  CALLBACK:
            file_name: String = file path
        Reads file
        Returns file text
    */
    app.global::<BackendTextEditor>().on_read_file(|file_name| {
        if file_name == "err".to_string(){
            eprintln!("Error opening text file");
            return "".to_string().into();
        }
        let mut text = "".to_string();
        match file_management::read_file(file_name.as_str()) {
            Ok(txt) => text = txt,
            Err(e) => eprintln!("Error loading file: {}", e),
        }
        return text.to_string().into();
    });

    /*  CALLBACK:
            new_size: String = new font size
            old_font: i32 = old font size
        Returns new_size as i32 if new_size is a number between 1 & 256
    */
    app.global::<BackendTextEditor>().on_set_font_size(|new_size, old_font| {
        let mut numeric = true;
        let mut font:i32 = 0;
        for ch in new_size.chars(){
            font = font * 10;
            if !ch.is_numeric(){
                numeric = false;
                break;
            }
            else{
                font += ch.to_digit(10).unwrap() as i32;
            }
        }
        if !numeric{
            font = old_font;
        }
        if font > 256{
            return 256;
        }
        if font <= 0{
            return 1;
        }
        return font;
    });


    app.run();

    Ok(())

}