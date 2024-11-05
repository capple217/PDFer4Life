slint::include_modules!();

pub fn defGlobalCallbacks(){
    let ui = TextEditor::new().unwrap();
    println!("ran");

    // ui.global::<BackendTextEditor>().on_save_file(|file_name, text| {
    //     println!("ran2");
    //     println!("here is the text for {}: {}", file_name, text);
    // });

    ui.global::<BackendPDF>().on_open_pdf(|file_name| {

    });
}