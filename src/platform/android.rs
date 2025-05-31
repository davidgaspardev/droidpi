use crate::resize::Resize;
use std::{fs, io::ErrorKind};

use super::Platform;

pub struct AndroidPlatform {
    resize: Resize,
}

impl AndroidPlatform {
    pub fn new(resize: Resize) -> Box<dyn Platform> {
        Box::new(AndroidPlatform { resize: resize })
    }
}

impl Platform for AndroidPlatform {
    fn create_images(&self, out_dir: &str) -> Result<(), String> {
        return match fs::create_dir_all(out_dir) {
            Ok(_) => {
                self.resize.create_mdpi(&format!("{}/mipmap-mdpi", out_dir));
                self.resize.create_hdpi(&format!("{}/mipmap-hdpi", out_dir));
                self.resize
                    .create_xhdpi(&format!("{}/mipmap-xhdpi", out_dir));
                self.resize
                    .create_xxhdpi(&format!("{}/mipmap-xxhdpi", out_dir));
                self.resize
                    .create_xxxdhpi(&format!("{}/mipmap-xxxhdpi", out_dir));

                Ok(())
            }
            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => {
                    eprintln!("Directory already exists: {}", out_dir);
                    Err(format!("Directory already exists: {}", out_dir))
                }
                _ => Err(format!("Error creating directory: {}", err)),
            },
        };
    }
}
