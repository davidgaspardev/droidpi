use super::Platform;
use crate::{context::Context, resize::Resize};
use std::{fs, io::ErrorKind};

pub struct FlutterPlatform {
    resize: Resize,
}

impl FlutterPlatform {
    pub fn new(resize: Resize) -> Box<dyn Platform> {
        Box::new(FlutterPlatform { resize: resize })
    }
}

impl Platform for FlutterPlatform {
    fn create_images(&self, ctx: &Context) -> Result<(), String> {
        let out_dir = ctx.get_arg_out_dir();

        return match fs::create_dir_all(out_dir) {
            Ok(_) => {
                self.resize.create_mdpi(&format!("{}/1.5x", out_dir));
                self.resize.create_hdpi(&format!("{}/2.0x", out_dir));
                self.resize.create_xhdpi(&format!("{}/3.0x", out_dir));
                self.resize.create_xxhdpi(&format!("{}/4.0x", out_dir));
                self.resize.create_xxxhdpi(&format!("{}", out_dir));

                Ok(())
            }
            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => {
                    eprintln!("Directory already exists: {}", out_dir);
                    return Err(format!("Directory already exists: {}", out_dir));
                }
                _ => panic!("Error creating directory: {}", err),
            },
        };
    }
}
