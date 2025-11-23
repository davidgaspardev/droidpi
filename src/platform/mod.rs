use crate::{context::Context, resize::Resize};

mod android;
mod flutter;

pub struct PlatformFactory;

impl PlatformFactory {
    pub fn get_platform_resize(
        platform: &str,
        resize: Resize,
    ) -> Result<Box<dyn Platform>, String> {
        if platform.eq("android") {
            return Ok(android::AndroidPlatform::new(resize));
        }

        if platform.eq("flutter") {
            return Ok(flutter::FlutterPlatform::new(resize));
        }

        Err(format!("Platform '{}' not found", platform))
    }
}

pub trait Platform {
    fn create_images(&self, ctx: &Context) -> Result<(), String>;
}
