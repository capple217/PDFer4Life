// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
use pdfium_render::prelude::*;
// use std::error::Error;

slint::include_modules!();
use slint::VecModel;
mod interface;
mod txt_file;
// mod pdf_renderer;
// use serde::{Deserialize, Serialize};
use serde_json::Result; // , Value}
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use std::sync::{Arc, Mutex};
use std::env;
// use std::{clone, vec};

fn main() -> Result<()> {
    //ideally result should also have: Result<(), slint::PlatformError>

    // let target_os = env::consts::OS;
    // let lib_path = match target_os.as_str() {
    //     "windows" => "pdfium-windows.dll",
    //     "linux" => "libpdfium.so",
    //     "macos" => "pdfium-mac.dylib",
    //     _ => panic!("Unsupported OS"),
    // };

    // let out_dir = env::var("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join(lib_path);
    // fs::copy(lib_path, dest_path).expect("Failed to copy Pdfium binary");


    // Application window -- define all global callbacks on this window
    let app = App::new().unwrap();

    // Initializes the file manager with local data if available
    let mut initial_file_manager = interface::FileManager::new();
    match txt_file::read_file("database.json") {
        Ok(data) => {
            if data != "" {
                initial_file_manager.set_files(serde_json::from_str(data.as_str())?)
            }
        }
        Err(_) => (),
    };

    let file_manager = Arc::new(Mutex::new(initial_file_manager));

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // CALLBACKS USED IN OPENING PAGE:
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////

    /*  CALLBACK:
        Prompts user to select PDF, then sets the active page to split-page
    */
    app.global::<AppService>().on_open_file({
        let app_weak = app.as_weak();
        let cloned_file_manager = file_manager.clone();
        move || {
            let app = app_weak.unwrap();
            let mut file_manager = cloned_file_manager.lock().unwrap();
            if file_manager.add_new_file() {
                app.set_active_page(1);
            }
        }
    });

    /*  CALLBACK:
        User selected PDF from recents, then sets the active page to split-page
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
            file_manager.set_cur_file_info(file_path.to_string());
        }
    });

    /*  CALLBACK:
        Returns all previously opened PDFs as slint vector for use in opening-page recent pdf buttons
    */
    app.global::<AppService>().on_get_recent_files({
        let cloned_file_manager = file_manager.clone();
        move || {
            let file_manager = cloned_file_manager.lock().unwrap();
            let mut recent_list = Vec::new();

            for a_file in file_manager.get_files().iter() {
                recent_list.push((a_file.get_name().into(), a_file.get_filepath().into()));
            }

            //let my_vec : Vec<(slint::SharedString, slint::SharedString)> = recent_list.into_iter().map(Into::into).collect();
            let model = slint::ModelRc::new(VecModel::from(recent_list));

            return model;
        }
    });

    /* CALLBACK:
       Returns the number of previously opened PDFs
    */
    app.global::<AppService>().on_get_num_recent_files({
        let cloned_file_manager = file_manager.clone();
        move || {
            let mut count = 0;
            let file_manager = cloned_file_manager.lock().unwrap();

            for _a_file in file_manager.get_files().iter() {
                count += 1;
            }
            return count;
        }
    });

    /* CALLBACK:
       Returns trimmed file name if name exceeds max length
    */
    let max_name_len = 15;
    app.global::<AppService>().on_trim_file_name(move |name| {
        if name.len() > max_name_len as usize {
            let mut new_name: String = name[0..max_name_len - 5].to_string();
            new_name += "...pdf";
            return new_name.into();
        }
        return name.into();
    });

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // CALLBACKS USED IN PDF RENDERING:
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////

    /*  CALLBACK:
       Renders current page of PDF and returns as Slint image
    */
    app.global::<BackendPDF>().on_display({
        let cloned_file_manager = file_manager.clone();
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

    /* CALLBACK:
       Navigates to the previous page in the pdf file
    */
    app.global::<BackendPDF>().on_navigate_previous({
        let cloned_file_manager = file_manager.clone();
        move || {
            let mut file_manager = cloned_file_manager.lock().unwrap();
            let num = file_manager.get_cur_page();
            if num > 0 {
                file_manager.set_cur_page(num - 1);
            }
        }
    });

    /*  CALLBACK:
       Navigates to the next page in the pdf file
    */
    app.global::<BackendPDF>().on_navigate_next({
        let cloned_file_manager = file_manager.clone();
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

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // CALLBACKS USED IN TEXT EDITOR:
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////

    /*  CALLBACK:
        Prompt user to select txt file and returns path as String
    */
    app.global::<BackendTextEditor>()
        .on_open_text_file(|| txt_file::open_file_txt().into());

    /*  CALLBACK:
        Saves text to specified file path (file_name)
    */
    app.global::<BackendTextEditor>().on_save_file(
        |file_name, text| match txt_file::write_to_file(file_name.as_str(), text.as_str()) {
            Ok(_) => println!("File Saved"),
            Err(e) => eprintln!("Error saving file: {}", e),
        },
    );

    /*  CALLBACK:
        Returns text at path (file_name) as String
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


    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // GENERAL APPLICATION CALLBACKS:
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////
    
    /* CALLBACK:
       Saves local data when application window is closed
    */
    app.window().on_close_requested({
        let cloned_file_manager = file_manager.clone();
        move || {
            let mut file_manager = cloned_file_manager.lock().unwrap();
            file_manager.add_file();
            let files = file_manager.get_files();

            let json = serde_json::to_string(&files).unwrap();

            match txt_file::write_to_file("database.json", json.as_str()) {
                Ok(_) => println!("File Saved"),
                Err(e) => eprintln!("Error saving file: {}", e),
            }

            println!("the json is {}", json);
            slint::CloseRequestResponse::HideWindow
        }
    });

    let _ = app.run();

    Ok(())
}
