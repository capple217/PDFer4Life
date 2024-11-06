slint::include_modules!();
use slint::Weak;

pub fn defGlobalCallbacks(app){
    // let app = app_weak.unwrap();

    app.global::<BackendTextEditor>().on_save_file(|file_name, text| {
        println!("ran3");
        println!("here is the text for {}: {}", file_name, text);
    });

}