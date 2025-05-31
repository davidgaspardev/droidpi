use crate::resize::Resize;

mod android;
mod flutter;

pub struct PlatformFactory;

impl PlatformFactory {
    pub fn get_platform_resize(platform: &str, resize: Resize) -> Box<dyn Platform> {
        if platform.eq("android") {
            return android::AndroidPlatform::new(resize);
        }

        if platform.eq("flutter") {
            return flutter::FlutterPlatform::new(resize);
        }

        panic!("Platform not found")
    }
}

pub trait Platform {
    fn create_images(&self, out_dir: &str) -> Result<(), String>;
}
