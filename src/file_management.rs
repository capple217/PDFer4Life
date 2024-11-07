use native_dialog::FileDialog;
use std::fs::File;
use std::io::Write;
use std::io::Result;
use std::fs;

#[derive(Default)]
pub struct Files;

impl Files {
    pub fn open_file() {
        if let Some(path) = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .show_open_single_file()
        .unwrap() {
            println!("Selected file: {:?}", path);
        }
    }
    pub fn open_file_txt() {
        if let Some(path) = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PNG Image", &["txt"])
        .show_open_single_file()
        .unwrap() {
            println!("Selected file: {:?}", path);
        }
    }
}

pub fn write_to_file(filename: &str, text: &str) -> Result<()> {
    let mut file = File::create(filename)?; // Opens a file in write-only mode
    file.write_all(text.as_bytes())?; // Writes the string as bytes to the file
    Ok(())
}

pub fn read_file(filename: &str) -> Result<String> {
    fs::read_to_string(filename)
}