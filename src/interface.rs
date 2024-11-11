use native_dialog::FileDialog;
use std::time::{SystemTime, UNIX_EPOCH};
// Create a map (name : file path)
// functionality to rename, delete, and view all files
// last read

#[derive(Default)]

pub struct FileInfo {
    name: String, 
    filepath: String,
    last_read: u64,
    page_num: u64,
}

impl FileInfo {
fn new(filename: &str, name: &str) -> Self {
        let now = SystemTime::now();
        //.duration_since(UNIX_EPOCH).as_days();
        Self {
            name: name.to_string(),
            filepath: filename.to_string(),
            last_read: now,
            page_num: 0,
        }
    }
}

pub struct FileManager{
    files: Vec<FileInfo>
}

impl FileManager{
    pub fn new() -> Self {
        Self {files: Vec::new()}
    }
    
    pub fn add_file(&mut self) {
        
        //open file from system
        if let Some(file_path) = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PDF File", &["pdf"])
        .show_open_single_file()
        .unwrap() {
            let name = "NaN";
            let file = FileInfo::new(file_path, name);
            self.files.push(file);
            println!("Selected file: {:?}", file_path);
        }
    }
    
    fn delete_file(&mut self, name: &str) {
        if let Some(pos) = self.files.iter().position(|file| file.name = name) {
            self.files.remove(pos);
        }
    }

    fn rename(&mut self, new_name: &str, old_name: &str) {
        if let Some(file) = self.files.iter_mut().find(|file| file.name = old_name) {
            file.name = new_name.to_string();
        }
    }
}
