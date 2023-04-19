use image::{DynamicImage, GenericImageView};

const FACTOR_XXXHDPI: f32 = 4.0;
const FACTOR_XXHDPI: f32 = 3.0;
const FACTOR_XHDPI: f32 = 2.0;
const FACTOR_HDPI: f32 = 1.5;
const FACTOR_MDPI: f32 = 1.0;

pub struct Resize {
    xxxhdpi_img: DynamicImage,
    name: String,
}

impl Resize {
    pub fn new(xxxhdpi_image: DynamicImage, name: String) -> Self {
        Resize {
            xxxhdpi_img: xxxhdpi_image,
            name: name
        }
    }

    pub fn create_xxhdpi(&self) {
        let (width, height) = self.xxxhdpi_img.dimensions();

        let new_width = (((width as f32) * FACTOR_XXHDPI) / FACTOR_XXXHDPI) as u32;
        let new_height = (((height as f32) * FACTOR_XXHDPI) / FACTOR_XXXHDPI) as u32;

        let xxhdpi_image = self.xxxhdpi_img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        xxhdpi_image.save(format!("data/{}@3.0x.png", self.name)).unwrap();
    }

    pub fn create_xhdpi(&self) {
        let (width, height) = self.xxxhdpi_img.dimensions();

        let new_width = (((width as f32) * FACTOR_XHDPI) / FACTOR_XXXHDPI) as u32;
        let new_height = (((height as f32) * FACTOR_XHDPI) / FACTOR_XXXHDPI) as u32;

        let xhdpi_image = self.xxxhdpi_img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        xhdpi_image.save(format!("data/{}@2.0x.png", self.name)).unwrap();
    }

    pub fn create_hdpi(&self) {
        let (width, height) = self.xxxhdpi_img.dimensions();

        let new_width = (((width as f32) * FACTOR_HDPI) / FACTOR_XXXHDPI) as u32;
        let new_height = (((height as f32) * FACTOR_HDPI) / FACTOR_XXXHDPI) as u32;

        let hdpi_image = self.xxxhdpi_img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        hdpi_image.save(format!("data/{}@1.5x.png", self.name)).unwrap();
    }

    pub fn create_mdpi(&self) {
        let (width, height) = self.xxxhdpi_img.dimensions();

        let new_width = (((width as f32) * FACTOR_MDPI) / FACTOR_XXXHDPI) as u32;
        let new_height = (((height as f32) * FACTOR_MDPI) / FACTOR_XXXHDPI) as u32;

        let mdpi_image = self.xxxhdpi_img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        mdpi_image.save(format!("data/{}@1.0x.png", self.name)).unwrap();
    }
}
