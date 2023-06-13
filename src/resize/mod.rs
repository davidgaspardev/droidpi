use image::{DynamicImage, GenericImageView};
use std::fs;

const FACTOR_XXXHDPI: f32 = 4.0;
const FACTOR_XXHDPI: f32 = 3.0;
const FACTOR_XHDPI: f32 = 2.0;
const FACTOR_HDPI: f32 = 1.5;
const FACTOR_MDPI: f32 = 1.0;

pub struct Resize {
    xxxhdpi_img: DynamicImage,
    name: String,
    width: u32,
    height: u32,
}

impl Resize {
    pub fn new(xxxhdpi_image: DynamicImage, name: String) -> Self {
        let (width, height) = xxxhdpi_image.dimensions();
        
        Resize {
            xxxhdpi_img: xxxhdpi_image,
            name,
            width,
            height
        }
    }

    pub fn create_xxxdhpi(&self, directory: &String) {
        if !fs::metadata(&directory).is_ok() {
            match fs::create_dir(&directory) {
                Ok(_) => println!("Directory created successfully: {}", directory),
                Err(err) => println!("Failed to create directory: {}", err),
            }
        }
        
        self.xxxhdpi_img.save(format!("{}/{}.png", directory, self.name)).unwrap();
    }

    pub fn create_xxhdpi(&self, directory: &String) {
        if !fs::metadata(&directory).is_ok() {
            match fs::create_dir(&directory) {
                Ok(_) => println!("Directory created successfully: {}", directory),
                Err(err) => println!("Failed to create directory: {}", err),
            }
        }

        let new_width = (((self.width as f32) * FACTOR_XXHDPI) / FACTOR_XXXHDPI) as u32;
        let new_height = (((self.height as f32) * FACTOR_XXHDPI) / FACTOR_XXXHDPI) as u32;

        let xxhdpi_image = self.xxxhdpi_img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        xxhdpi_image.save(format!("{}/{}.png", directory, self.name)).unwrap();
    }

    pub fn create_xhdpi(&self, directory: &String) {
        if !fs::metadata(&directory).is_ok() {
            match fs::create_dir(&directory) {
                Ok(_) => println!("Directory created successfully: {}", directory),
                Err(err) => println!("Failed to create directory: {}", err),
            }
        }

        let new_width = (((self.width as f32) * FACTOR_XHDPI) / FACTOR_XXXHDPI) as u32;
        let new_height = (((self.height as f32) * FACTOR_XHDPI) / FACTOR_XXXHDPI) as u32;

        let xhdpi_image = self.xxxhdpi_img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        xhdpi_image.save(format!("{}/{}.png", directory, self.name)).unwrap();
    }

    pub fn create_hdpi(&self, directory: &String) {
        if !fs::metadata(&directory).is_ok() {
            match fs::create_dir(&directory) {
                Ok(_) => println!("Directory created successfully: {}", directory),
                Err(err) => println!("Failed to create directory: {}", err),
            }
        }

        let new_width = (((self.width as f32) * FACTOR_HDPI) / FACTOR_XXXHDPI) as u32;
        let new_height = (((self.height as f32) * FACTOR_HDPI) / FACTOR_XXXHDPI) as u32;

        let hdpi_image = self.xxxhdpi_img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        hdpi_image.save(format!("{}/{}.png", directory, self.name)).unwrap();
    }

    pub fn create_mdpi(&self, directory: &String) {
        if !fs::metadata(&directory).is_ok() {
            match fs::create_dir(&directory) {
                Ok(_) => println!("Directory created successfully: {}", directory),
                Err(err) => println!("Failed to create directory: {}", err),
            }
        }

        let new_width = (((self.width as f32) * FACTOR_MDPI) / FACTOR_XXXHDPI) as u32;
        let new_height = (((self.height as f32) * FACTOR_MDPI) / FACTOR_XXXHDPI) as u32;

        let mdpi_image = self.xxxhdpi_img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        mdpi_image.save(format!("{}/{}.png", directory, self.name)).unwrap();
    }
}
