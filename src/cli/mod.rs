pub mod flag;
pub use flag::CliArgs;

use clap::Parser;
use std::path::Path;

pub fn get_arguments() -> Result<CliArgs, String> {
    let args = CliArgs::parse();
    
    // Validate the arguments
    validate_arguments(&args)?;
    
    Ok(args)
}

/// Validate the parsed arguments
fn validate_arguments(args: &CliArgs) -> Result<(), String> {
    // Validate source file exists and is an image
    if !args.src.is_file() {
        return Err(format!("Source file does not exist: {:?}", args.src));
    }
    
    if !is_path_imagefile(&args.src) {
        return Err(format!("Source file must be a .png, .jpg, or .jpeg file: {:?}", args.src));
    }
    
    // Validate output directory exists
    if !args.out_dir.is_dir() {
        return Err(format!("Output directory does not exist: {:?}", args.out_dir));
    }
    
    Ok(())
}

fn is_path_imagefile(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        ext_str == "png" || ext_str == "jpg" || ext_str == "jpeg"
    } else {
        false
    }
}
