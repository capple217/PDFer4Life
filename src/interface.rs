use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};
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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_filepath(&self) -> String {
        self.filepath.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileManager {
    files: Vec<FileInfo>,
    cur_file_path: String,
    cur_file_page: u16,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            cur_file_path: "../assets/blank.pdf".to_string(),
            cur_file_page: 0,
        }
    }

    pub fn add_file(&mut self) -> bool {
        //open file from system
        if let Some(file_path) = FileDialog::new()
            .set_location("~/Desktop")
            .add_filter("PDF File", &["pdf"])
            .show_open_single_file()
            .unwrap()
        {
            let name = file_path.file_name().unwrap().to_str().unwrap();
            let file = FileInfo::new(file_path.to_str().unwrap(), name);
            self.files.push(file);
            println!("Selected file: {:?}", file_path);
            self.set_cur_path(file_path.to_str().unwrap().to_string());
            return true;
        } else {
            return false;
        }
    }

    pub fn get_cur_path(&mut self) -> Result<String, String> {
        if self.cur_file_path == "".to_string(){
            return Err("cant".to_string());
        }
        return Ok(self.cur_file_path.clone());
    }

    pub fn set_cur_path(&mut self, str: String) {
        self.cur_file_path = str;
    }

    pub fn get_cur_page(&mut self) -> u16 {
        return self.cur_file_page;
    }

    pub fn set_cur_page(&mut self, num:u16) {
        self.cur_file_page = num;
    }


    // fn delete_file(&mut self, name: &str) {
        // if let Some(pos) = self.files.iter().position(|file| file.name = name) {
        //     self.files.remove(pos);
        // }
    // }

    // fn rename(&mut self, new_name: &str, old_name: &str) {
        // if let Some(file) = self.files.iter_mut().find(|file| file.name = old_name) {
        //     file.name = new_name.to_string();
        // }
    // }

    pub fn getFiles(&self) -> &Vec<FileInfo> {
        return &self.files;
    }

    pub fn setFiles(&mut self, data: Vec<FileInfo>) {
        self.files = data;
    }
}
