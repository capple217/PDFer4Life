use native_dialog::FileDialog;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
// Create a map (name : file path)
// functionality to rename, delete, and view all files
// last read

#[derive(Default)]

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    name: String, 
    filepath: String,
    //last_read: SystemTime,
    page_num: u64,
    //attached_txt: String
}

impl FileInfo {
fn new(filename: &str, name: &str) -> Self {
        //let now = SystemTime::now();
        Self {
            name: name.to_string(),
            filepath: filename.to_string(),
            //last_read: now,
            page_num: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileManager{
    files: Vec<FileInfo> 
}

impl FileManager{
    pub fn new() -> Self {
        Self {files: Vec::new()}
    }
    
    pub fn add_file(&mut self) -> bool {
        
        //open file from system
        if let Some(file_path) = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PDF File", &["pdf"])
        .show_open_single_file()
        .unwrap() {
            let name = file_path.file_name().unwrap().to_str().unwrap();
            let file = FileInfo::new(file_path.to_str().unwrap(), name);
            self.files.push(file);
            println!("Selected file: {:?}", file_path);
            return true;
        } else {
            return false;
        }
    }
    
    fn delete_file(&mut self, name: &str) {
        // if let Some(pos) = self.files.iter().position(|file| file.name = name) {
        //     self.files.remove(pos);
        // }
    }

    fn rename(&mut self, new_name: &str, old_name: &str) {
        // if let Some(file) = self.files.iter_mut().find(|file| file.name = old_name) {
        //     file.name = new_name.to_string();
        // }
    }

    pub fn getFiles(&self) -> &Vec<FileInfo>{
        return &self.files;
    }
}


