use std::env;
use std::fs;
use std::path::Path;

fn main() {
    slint_build::compile("ui/app.slint").expect("Slint build failed");



    // Get the build output directory
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    println!("out_dir: {}", out_dir);

    // Define the source file path (relative to the project root)
    let target_os = env::consts::OS;
    println!("target_os: {}", target_os);
    let filename;
    let source_file;
    match target_os {
        "windows" => {source_file = "./lib/pdfium.dll"; filename="pdfium.dll";},
        "linux" => {source_file = "./lib/libpdfium.so"; filename="libpdfium.so";},
        "macos" => {source_file = "./lib/libpdfium.dylib"; filename="libpdfium.dylib";},
        _ => panic!("Unsupported OS"),
    };

    // Construct the destination file path
    let dir_path = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap().join(filename);
    println!("src: {}, outdir:{}", source_file, out_dir);

    let _ = fs::rename(source_file, dir_path);
}
