use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};
use std::result::Result;


#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    name: String,
    filepath: String,
    cur_file_page: u16,
    //attached_txt: String
}

impl FileInfo {
    fn new(filename: &str, name: &str, cur_file_page: u16) -> Self {
        Self {
            name: name.to_string(),
            filepath: filename.to_string(),
            cur_file_page
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_filepath(&self) -> String {
        self.filepath.clone()
    }

    pub fn get_cur_page(&mut self) -> u16 {
        return self.cur_file_page;
    }

    pub fn set_cur_page(&mut self, num:u16) {
        self.cur_file_page = num;
        println!("new page: {}", self.cur_file_page);
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileManager {
    files: Vec<FileInfo>,
    cur_file_info: FileInfo,
    cur_file_path: String,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            cur_file_info: FileInfo::new("../assets/blank.pdf", "blank.pdf", 0),
            cur_file_path: "../assets/blank.pdf".to_string(),
        }
    }

    pub fn add_new_file(&mut self) -> bool {
        //open file from system
        if let Some(file_path) = FileDialog::new()
            .set_location("~/Desktop")
            .add_filter("PDF File", &["pdf"])
            .show_open_single_file()
            .unwrap()
        {
            let name = file_path.file_name().unwrap().to_str().unwrap();
            self.cur_file_info = FileInfo::new(file_path.to_str().unwrap(), name, 0);
            println!("Selected file: {:?}", file_path);
            self.set_cur_path(file_path.to_str().unwrap().to_string());
            return true;
        } else {
            return false;
        }
    }

    pub fn add_file(& mut self) -> bool {
        //open file from system
        if self.cur_file_path != "../assets/blank.pdf".to_string() {
            self.files.insert(0, FileInfo::new(&self.cur_file_info.get_filepath(), &self.cur_file_info.get_name(), self.cur_file_info.get_cur_page()));
            return true;
        } else {
            return false;
        }
    }

    pub fn set_cur_file_info(&mut self, str: String) {
        let index = self.files.iter().position(|n| n.get_filepath() == str).unwrap();
        self.cur_file_info = self.files.remove(index);
    }

    pub fn get_cur_file_info(&mut self) -> &mut FileInfo {
        return &mut self.cur_file_info;
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

    pub fn get_files(&self) -> &Vec<FileInfo> {
        return &self.files;
    }

    pub fn set_files(&mut self, data: Vec<FileInfo>) {
        self.files = data;
    }
}
