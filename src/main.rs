extern crate image;

use std::fs;
use std::io::ErrorKind;

mod cli;
mod resize;

fn main() {
    println!("Welcome to droiDPI");

    let result = cli::get_arguments();

    if let Err(err) = result {
        eprintln!("{}", err);
        return
    }

    if let Ok(setting) = result {
        println!("setting: {:?}", setting);
        println!("setting[IMAGE_PATH]: {}", setting[cli::IMAGE_PATH]);
        println!("setting[FLUTTER_PATH]: {}", setting[cli::FLUTTER_PATH]);
        println!("setting[IMAGE_NAME]: {}", setting[cli::IMAGE_NAME]);

        let image_path = &setting[cli::IMAGE_PATH];
        let flutter_path = &setting[cli::FLUTTER_PATH];
        let image_name = &setting[cli::IMAGE_NAME];

        if let Ok(img) = image::open(image_path) {
            let img_resize = resize::Resize::new(img, image_name.to_string());

            let all_path = [
                format!("{}", flutter_path),
                format!("{}/1.5x", flutter_path),
                format!("{}/2.0x", flutter_path),
                format!("{}/3.0x", flutter_path),
                format!("{}/4.0x", flutter_path)
            ];

            match fs::create_dir_all(flutter_path) {
                Ok(_) => {
                    img_resize.create_mdpi(&all_path[0]);
                    img_resize.create_hdpi(&all_path[1]);
                    img_resize.create_xhdpi(&all_path[2]);
                    img_resize.create_xxhdpi(&all_path[3]);
                    img_resize.create_xxxdhpi(&all_path[4]);
                },
                Err(err) => match err.kind() {
                    ErrorKind::AlreadyExists => eprintln!("Directory already exists: {}", flutter_path),
                    _ => panic!("Error creating directory: {}", err)
                }
            }
        }
    }
}