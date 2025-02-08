use std::{borrow::Cow, path::Path};

use arboard::Clipboard;
use image::RgbaImage;

pub fn save_to_clipboard(image: RgbaImage) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_image(arboard::ImageData {
        width: image.width() as usize,
        height: image.height() as usize,
        bytes: Cow::Owned(image.as_raw().to_vec()),
    })?;
    Ok(())
}

pub fn save_as_file(image: RgbaImage, file_name: String) -> Result<(), Box<dyn std::error::Error>> {
    image.save(get_available_filename(&file_name))?;
    Ok(())
}

fn get_available_filename(original_path: &String) -> String {
    if !Path::new(original_path).exists() {
        return original_path.to_string();
    }

    let path = Path::new(original_path);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let ext = path.extension().unwrap().to_str().unwrap();

    let mut counter = 1;
    loop {
        let new_name = format!("{}({}).{}", stem, counter, ext);
        if !Path::new(&new_name).exists() {
            return new_name;
        }
        counter += 1;
    }
}
