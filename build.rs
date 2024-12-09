use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io;

fn main() {
    slint_build::compile("ui/slint-exports.slint").expect("Slint build failed");



    // Get the build output directory
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    println!("out_dir: {}", out_dir);

    println!("file: {}", fs::read_to_string("./lib/test.txt").unwrap());

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
    let dir_path = Path::new(&out_dir).parent().unwrap().join(filename);
    println!("src: {}, outdir:{}", source_file, out_dir);

    fs::rename(source_file, dir_path);

    // let mut file = File::create(filename).unwrap(); 
    // let data = fs::read_to_string(&source_file).unwrap();
    // file.write_all(data.as_bytes()); 

    // // Copy the file
    // fs::copy(source_file, &dest_path)
    //     .unwrap_or_else(|e: std::io::Error| panic!("Failed to copy {}: {}", source_file, e));

    // println!("cargo:rerun-if-changed={}", source_file); // Ensures the build script reruns if the source file changes
}

// let target_os = env::consts::OS;
// let lib_path = match target_os {
//     "windows" => "lib/pdfium.dll",
//     "linux" => "lib/libpdfium.so",
//     "macos" => "lib/libpdfium.dylib",
//     _ => panic!("Unsupported OS"),
// };

// let out_dir = env::var("OUT_DIR").unwrap();
// let dest_path = Path::new(&out_dir).join(lib_path);
// fs::copy(lib_path, dest_path).expect("Failed to copy Pdfium binary");
