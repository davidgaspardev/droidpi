//! Help module for DroiDPI
//!
//! This module provides functions to display help messages and usage instructions.

/// Display the help message with usage instructions and examples
pub fn show_help(name: &str, version: &str) {
    println!("{} v{}", name, version);
    println!("\nDroiDPI is a command-line tool for resizing images to different screen densities commonly used in mobile application development.");
    println!("It simplifies the process of generating multiple sizes of icons for Flutter and native Android projects.");
    println!("\nUSAGE:");
    println!("  droidpi --src <image_path> --outdir <directory_path> --name <image_name> --platform <flutter|android>");
    println!("\nOPTIONS:");
    println!("  --src <image_path>         Path to the input image file (.png, .jpg, or .jpeg)");
    println!("  --outdir <directory_path>  Base directory where resized images will be stored");
    println!("  --name <image_name>        Desired name for the resized images");
    println!("  --platform <platform>      Target platform (flutter or android)");
    println!("  --version                  Show the version information");
    println!("  --help                     Display this help message");
    println!("\nEXAMPLES:");
    println!("  droidpi --src logo.png --outdir ./assets --name logo --platform flutter");
    println!("  droidpi --src icon.png --outdir ./res --name ic_launcher --platform android");
    println!("\nDENSITIES:");
    println!("  The tool resizes images to five different densities:");
    println!("  - mdpi (baseline)");
    println!("  - hdpi (1.5x)");
    println!("  - xhdpi (2.0x)");
    println!("  - xxhdpi (3.0x)");
    println!("  - xxxhdpi (4.0x)");
    println!("\nDIRECTORY STRUCTURE:");
    println!("  For Flutter:");
    println!("    .../my_icon.png       (mdpi baseline)");
    println!("    .../1.5x/my_icon.png  (hdpi)");
    println!("    .../2.0x/my_icon.png  (xhdpi)");
    println!("    .../3.0x/my_icon.png  (xxhdpi)");
    println!("    .../4.0x/my_icon.png  (xxxhdpi)");
    println!("\n  For Android:");
    println!("    .../mipmap-mdpi/my_icon.png");
    println!("    .../mipmap-hdpi/my_icon.png");
    println!("    .../mipmap-xhdpi/my_icon.png");
    println!("    .../mipmap-xxhdpi/my_icon.png");
    println!("    .../mipmap-xxxhdpi/my_icon.png");
    println!("\nTROUBLESHOOTING:");
    println!("  Common errors and solutions:");
    println!("  • \"Failed to get src argument\" - Ensure --src points to a valid image file (.png, .jpg, .jpeg)");
    println!(
        "  • \"Failed to get out dir argument\" - Ensure --outdir points to an existing directory"
    );
    println!("  • \"Failed to get platform argument\" - Specify either 'flutter' or 'android'");
    println!("  • \"Failed to get name argument\" - Provide a name for the output images");
    println!("  • \"Directory already exists\" - The output directory already contains densities folders");
    println!("  • \"Platform not found\" - Only 'flutter' and 'android' are supported platforms");
    println!("\nNOTES:");
    println!("  • The source image should preferably be in the highest resolution (xxxhdpi/4.0x)");
    println!("  • The tool will automatically create all necessary subdirectories");
    println!("  • All arguments are required except --version and --help");
}

/// Display a short usage message for error situations
pub fn show_short_help() {
    println!("Usage: droidpi --src <image_path> --outdir <directory_path> --name <image_name> --platform <flutter|android>");
    println!("Run with --help for more information.");
}

/// Display version information
pub fn show_version(name: &str, version: &str) {
    println!("{} v{}", name, version);
}
