use native_dialog::FileDialog;

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
}