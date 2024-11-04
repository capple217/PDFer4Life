
use std::time::{SystemTime, UNIX_EPOCH};
// Create a map (name : file path)
// functionality to rename, delete, and view all files
// last read


struct FileInfo {
    name: String, 
    filepath: String,
    last_read: u64,
    page_num: u64,
}

impl FileInfo {
fn new_file(filename: &str, name: &str) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).as_secs();
        Fileinfo {
            name,
            filename,
            last_read: now,
            page_num: 0,
        }
    }
}

struct FileManager{
    files: Vec<FileInfo>;
}

impl FileManager{
    fn new() -> Self {
        Self {files: Vec::new()}
   }
    
    fn add_file(&mut self, filename: &str, name: &str) {
        let file = FileInfo::new(filename, name);
        self.files.push(file);
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
