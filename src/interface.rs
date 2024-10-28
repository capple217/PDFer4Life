
// Create a map (name : file path)
// functionality to rename, delete, and view all files
// last read


struct FileInfo {
    name:Sring, 
    filepath: String,
    last_read: u64,
    page_num: u64,
}

fn new_file(filename: &str, name: &str) {
    //ADD functionality to know date and time
    stored_files.push(Fileinfo(
        name,
        filename,
        last_read: 0,
        page_num: 0,
    ));
}

struct FileManager{
    files: Vec<FileInfo>;
}

impl FileManager{
    fn new() -> Self {
        Self {files: Vec::new()}
    }
    
    fn delete_file(name: &str) {
        if let Some(pos) = stored_files.iter().position(|file| pos.name = name) {
            stored_files.remove(file);
        }
    }

    fn rename(new_name: &str, old_name: &str) {
        if let Some(file) = stored_files.iter_mut().find(|file| file.name = old_name) {
            file.name = new_name.to_string();
        }
    }
}