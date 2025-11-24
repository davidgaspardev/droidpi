use crate::resize::Resize;
use std::{fs, io::ErrorKind};

use super::Platform;

pub struct AndroidPlatform {
    resize: Resize,
    use_drawable: bool,
}

impl AndroidPlatform {
    pub fn new(resize: Resize, use_drawable: bool) -> Box<dyn Platform> {
        Box::new(AndroidPlatform { 
            resize: resize,
            use_drawable: use_drawable,
        })
    }
    
    fn get_dir_prefix(&self) -> &str {
        if self.use_drawable {
            "drawable"
        } else {
            "mipmap"
        }
    }
}

impl Platform for AndroidPlatform {
    fn create_images(&self, out_dir: &str) -> Result<(), String> {
        let prefix = self.get_dir_prefix();
        
        return match fs::create_dir_all(out_dir) {
            Ok(_) => {
                self.resize.create_mdpi(&format!("{}/{}-mdpi", out_dir, prefix));
                self.resize.create_hdpi(&format!("{}/{}-hdpi", out_dir, prefix));
                self.resize
                    .create_xhdpi(&format!("{}/{}-xhdpi", out_dir, prefix));
                self.resize
                    .create_xxhdpi(&format!("{}/{}-xxhdpi", out_dir, prefix));
                self.resize
                    .create_xxxhdpi(&format!("{}/{}-xxxhdpi", out_dir, prefix));

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
