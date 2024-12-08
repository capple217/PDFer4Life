// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
use pdfium_render::prelude::*;
use std::error::Error;

slint::include_modules!();
use slint::VecModel;
mod interface;
mod txt_file;
// mod pdf_renderer;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use std::sync::{Arc, Mutex};
use std::{clone, vec};

fn main() -> Result<()> {
    //ideally result should also have: Result<(), slint::PlatformError>
    /*
    Application Window
    */
    let app = App::new().unwrap();
    /*
    File Manager
    */

    let mut initial_file_manager = interface::FileManager::new();

    match txt_file::read_file("database.json") {
        Ok(data) => {
            if (data != "") {
                initial_file_manager.setFiles(serde_json::from_str(data.as_str())?)
            }
        }
        Err(e) => (),
    };

    let mut file_manager = Arc::new(Mutex::new(initial_file_manager));


    /*  CALLBACK:
        Prompts user selects PDF -> switch to text-editor pdf-render splitscreen page
    */
    app.global::<AppService>().on_open_file({
        let app_weak = app.as_weak();
        let cloned_file_manager = file_manager.clone();
        move || {
            let app = app_weak.unwrap();
            let mut file_manager = cloned_file_manager.lock().unwrap();
            if file_manager.add_file() {
                app.set_active_page(1);
            }
        }
    });

    /*  CALLBACK:
        Prompts user selects PDF from recents -> switch to text-editor pdf-render splitscreen page
    */
    app.global::<AppService>().on_open_recent_file({
        let app_weak = app.as_weak();
        let cloned_file_manager = file_manager.clone();
        move |file_path| {
            let app = app_weak.unwrap();
            let mut file_manager = cloned_file_manager.lock().unwrap();
            app.set_active_page(1);
            println!("{}", file_path.to_string());
            file_manager.set_cur_path(file_path.to_string());
        }
    });

    /*  CALLBACK:
        list recently opened PDFs
    */
    app.global::<AppService>().on_get_recent_files({
        let cloned_file_manager = file_manager.clone();
        move || {
            let file_manager = cloned_file_manager.lock().unwrap();
            let mut recent_list = Vec::new();

            for a_file in file_manager.getFiles().iter() {
                recent_list.push((a_file.get_name().into(), a_file.get_filepath().into()));
            }

            //let my_vec : Vec<(slint::SharedString, slint::SharedString)> = recent_list.into_iter().map(Into::into).collect();
            let model = slint::ModelRc::new(VecModel::from(recent_list));

            return model;
        }
    });

    /* CALLBACK: 
        Saves file data to local json when window is closed
        RETURNS Slint hidewindow
     */
    app.window().on_close_requested({
        let cloned_file_manager = file_manager.clone();
        move || {
            let file_manager = cloned_file_manager.lock().unwrap();
            let files = file_manager.getFiles();

            let json = serde_json::to_string(&files).unwrap();

            match txt_file::write_to_file("database.json", json.as_str()) {
                Ok(_) => println!("File Saved"),
                Err(e) => eprintln!("Error saving file: {}", e),
            }

            println!("the json is {}", json);
            slint::CloseRequestResponse::HideWindow
        }
    });

    //BACKEND PDF
    // pure callback navigate_previous();
    // pure callback navigate_next();
    app.global::<BackendPDF>().on_display({
        let mut cloned_file_manager = file_manager.clone();
        move || {
            let mut file_manager = cloned_file_manager.lock().unwrap();
            let current_page = file_manager.get_cur_page();
            let pdfium = Pdfium::default();
            let file_path = file_manager.get_cur_path().unwrap();
            let document = pdfium.load_pdf_from_file(file_path.as_str(), None).unwrap();
            let page = document.pages().get(current_page as u16).unwrap();
            let render_config = PdfRenderConfig::new()
                .set_target_width(2000)
                .set_maximum_height(2000);

            let image = page
                .render_with_config(&render_config)
                .unwrap()
                .as_image()
                .into_rgba8();

            let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                image.as_raw(),
                image.width(),
                image.height(),
            );

            Image::from_rgba8(buffer)
        }
    });

    app.global::<BackendPDF>().on_navigate_previous({
        let mut cloned_file_manager = file_manager.clone();
        move || {
            let mut file_manager = cloned_file_manager.lock().unwrap();
            let num = file_manager.get_cur_page();
            if num > 0 {
                file_manager.set_cur_page(num - 1);
            }
        }
    });

    app.global::<BackendPDF>().on_navigate_next({
        let mut cloned_file_manager = file_manager.clone();
        move || {
            let mut file_manager = cloned_file_manager.lock().unwrap();
            let pdfium = Pdfium::default();
            let file_path = file_manager.get_cur_path().unwrap();
            let document = pdfium.load_pdf_from_file(file_path.as_str(), None).unwrap();
            let num = file_manager.get_cur_page();
            if num + 1 < document.pages().len().into() {
                file_manager.set_cur_page(num + 1);
            }
        }
    });

    /*  CALLBACK:
        Prompt user to select txt file
        Returns path to txt file as String
    */
    app.global::<BackendTextEditor>()
        .on_open_text_file(|| txt_file::open_file_txt().into());

    /*  CALLBACK:
            file_name: String = file path
            text: String = text to save
        Saves text to specified file path
        Returns void
    */
    app.global::<BackendTextEditor>().on_save_file(
        |file_name, text| match txt_file::write_to_file(file_name.as_str(), text.as_str()) {
            Ok(_) => println!("File Saved"),
            Err(e) => eprintln!("Error saving file: {}", e),
        },
    );

    /*  CALLBACK:
            file_name: String = file path
        Reads file
        Returns file text
    */
    app.global::<BackendTextEditor>().on_read_file(|file_name| {
        if file_name == "err".to_string() {
            eprintln!("Error opening text file");
            return "".to_string().into();
        }
        let mut text = "".to_string();
        match txt_file::read_file(file_name.as_str()) {
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
    app.global::<BackendTextEditor>()
        .on_set_font_size(|new_size, old_font| {
            let mut numeric = true;
            let mut font: i32 = 0;
            for ch in new_size.chars() {
                font = font * 10;
                if !ch.is_numeric() {
                    numeric = false;
                    break;
                } else {
                    font += ch.to_digit(10).unwrap() as i32;
                }
            }
            if !numeric {
                font = old_font;
            }
            if font > 256 {
                return 256;
            }
            if font <= 0 {
                return 1;
            }
            return font;
        });

    app.run();

    Ok(())
}
