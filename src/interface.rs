
// Create a map (name : file path)
// functionality to rename, delete, and view all files
// last read

let mut stored_files: Vec<FileInfo> = Vec::new();

struct FileInfo {
    name: &str,
    filepath: str,
    last_read: &str,
    page_num: u64,
}

fn new_file(filename: &str, name: &str) {
    //ADD functionality to know date and time
    stored_files.push(Fileinfo(
        name,
        filename,
        last_read,
        page_num: 0,
    ));
}

fn delete_file(filename: &str);

fn rename(new_name: &str, old_name: &str);

fn view_files();
