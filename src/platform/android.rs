use crate::{cli::Flag, context::Context, resize::Resize};
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
    fn create_images(&self, ctx: &Context) -> Result<(), String> {
        let out_dir = ctx.get_arg_out_dir();
        let dir_base = if ctx.args.contains_key(Flag::UseDrawable.as_str()) {
            "drawable"
        } else {
            "mipmap"
        };
        let final_dir = format!("{}/{}", out_dir, dir_base);

        return match fs::create_dir_all(out_dir) {
            Ok(_) => {
                self.resize.create_mdpi(&format!("{}-mdpi", final_dir));
                self.resize.create_hdpi(&format!("{}-hdpi", final_dir));
                self.resize.create_xhdpi(&format!("{}-xhdpi", final_dir));
                self.resize.create_xxhdpi(&format!("{}-xxhdpi", final_dir));
                self.resize
                    .create_xxxhdpi(&format!("{}-xxxhdpi", final_dir));

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
