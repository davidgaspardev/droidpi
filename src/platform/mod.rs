use crate::resize::Resize;

mod android;
mod flutter;

pub struct PlatformFactory;

impl PlatformFactory {
    pub fn get_platform_resize(
        platform: &str,
        resize: Resize,
        use_drawable: bool,
    ) -> Result<Box<dyn Platform>, String> {
        if platform.eq("android") {
            return Ok(android::AndroidPlatform::new(resize, use_drawable));
        }

        if platform.eq("flutter") {
            // Note: use_drawable flag only applies to Android platform
            // Flutter uses a fixed directory structure (1.5x, 2.0x, 3.0x, 4.0x)
            return Ok(flutter::FlutterPlatform::new(resize));
        }

        Err(format!("Platform '{}' not found", platform))
    }
}

pub trait Platform {
    fn create_images(&self, out_dir: &str) -> Result<(), String>;
}
