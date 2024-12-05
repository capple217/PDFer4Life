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
    let mut path: String;


    /*  CALLBACK:
        Prompts user selects PDF -> switch to text-editor pdf-render splitscreen page
    */
    app.global::<AppService>().on_open_file({
        let app_weak = app.as_weak();
        let cloned_file_manager = file_manager.clone();
        move || {
            let app = app_weak.unwrap();
            match cloned_file_manager.lock().unwrap().add_file() {
                Ok(var1) => {path = var1;
                            app.set_active_page(1);
                },
                Err(var2) => path = var2.to_string() 
            }
        }
    });


    // Global variables:
    static PDF_DOCUMENT: Mutex<Option<PdfDocument<'static>>> = Mutex::new(None);
    static CURRENT_PAGE: Mutex<usize> = Mutex::new(0);

    fn initialize_pdf(file_path: &str) -> Result<(), PdfiumError> {
        let pdfium = Pdfium::default();
        let document = pdfium.load_pdf_from_file(file_path, None)?;
        *PDF_DOCUMENT.lock().unwrap() = Some(document);
        *CURRENT_PAGE.lock().unwrap() = 0;
        Ok(())
    }

    fn render_current_page() -> Option<Image> {
        let document = PDF_DOCUMENT.lock().unwrap();
        let current_page = CURRENT_PAGE.lock().unwrap();

        if let Some(ref doc) = *document {
            let page = doc.pages().get(current_page as u16).unwrap();
            let render_config = PdfRenderConfig::new().set_target_width(800).set_maximum_height(800);
            let image = page.render_with_config(&render_config).unwrap().as_image().into_rgba8();

            let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                image.as_raw(),
                image.width(),
                image.height(),
            );

            return Some(Image::from_rgba8(buffer));
        }

        None
    }
    
    //  Initialize pdf_renderer after given file file path
    let mut pdfer = pdf_renderer::PDFViewer::new(&path).unwrap();

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
        pdfer.navigate_previous();
    });


    app.global::<BackendPDF>().on_navigate_next(||{
        pdfer.navigate_next();
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