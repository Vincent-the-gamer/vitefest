use std::{
    fs::{create_dir, File}, 
    io::Write
};

use image::DynamicImage;

/**
 * Handle the texts
 */

pub fn create_folder(directory: &str) {
    match create_dir(directory) {
        Ok(_) => println!("Successfully created folder: {}", directory),
        Err(e) => println!("Error while creating folder: {}", e)
    }
}

// write texts into file
pub fn write_file(path: &str, content: &str) {
    let mut file = File::create(path).expect("Error while creating file");
    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("Successfully wrote file: {}", path),
        Err(e) => println!("Error while creating file: {}", e)
    }
}

pub fn read_file(path: &str) -> File {
    File::open(path).expect("Error reading file.")
}


/**
 * Handle the texts
 */

pub fn read_image(path: &str) -> DynamicImage {
    image::open(path).expect("Error while reading image.")
}

pub fn save_image(image: DynamicImage, path: &str) {
    match image.save(path) {
        Ok(_) => println!("Successfully drew image: {}", path),
        Err(e) => println!("Error while drawing image: {}", e)
    };
}

// Save images as byte array
pub fn save_bytes_as_image(bytes: &[u8], output_path: &str) {
    let picture = image::load_from_memory(bytes).expect("Read image bytes error!");
    match picture.save(output_path) {
        Ok(_) => println!("Successfully drew image: {}", output_path),
        Err(e) => println!("Error while drawing image: {}", e)
    };
}