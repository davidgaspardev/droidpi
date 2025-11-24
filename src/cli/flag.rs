use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// A command-line tool for resizing images to multiple screen densities for Flutter and native Android projects
#[derive(Parser, Debug)]
#[command(name = "droidpi")]
#[command(version)]
#[command(about = "DroiDPI is a command-line tool for resizing images to different screen densities commonly used in mobile application development.\nIt simplifies the process of generating multiple sizes of icons for Flutter and native Android projects.")]
#[command(long_about = None)]
pub struct CliArgs {
    /// Path to the input image file (.png, .jpg, or .jpeg)
    #[arg(long, value_name = "IMAGE_PATH")]
    pub src: PathBuf,

    /// Base directory where resized images will be stored
    #[arg(long = "outDir", value_name = "DIRECTORY_PATH")]
    pub out_dir: PathBuf,

    /// Desired name for the resized images
    #[arg(long, value_name = "IMAGE_NAME")]
    pub name: String,

    /// Target platform (flutter or android)
    #[arg(long, value_enum)]
    pub platform: Platform,

    /// Use drawable directories instead of mipmap for Android (only applies to android platform)
    #[arg(long, default_value_t = false)]
    pub use_drawable: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Platform {
    /// Flutter platform - generates images in 1.5x, 2.0x, 3.0x, 4.0x directories
    Flutter,
    /// Android platform - generates images in mipmap-* or drawable-* directories
    Android,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Flutter => "flutter",
            Platform::Android => "android",
        }
    }
}
