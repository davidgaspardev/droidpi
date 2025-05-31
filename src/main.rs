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
        return;
    }

    if let Ok(setting) = result {
        println!("setting: {:?}", setting);
        println!("setting[FLAG_SRC]: {}", setting[cli::FLAG_SRC]);
        println!("setting[FLUTTER_PATH]: {}", setting[cli::FLUTTER_PATH]);
        println!("setting[IMAGE_NAME]: {}", setting[cli::IMAGE_NAME]);

        let image_path = &setting[cli::FLAG_SRC];
        let out_dir = &setting[cli::FLAG_OUTDIR];
        let target = &setting[cli::FLAG_TARGET];
        let name = &setting[cli::FLAG_NAME];

         let all_path = get_paths_by_target(out_dir, target).expect("Error getting paths");

        if let Ok(img) = image::open(image_path) {
            let img_resize = resize::Resize::new(img, name.to_string());

            match fs::create_dir_all(out_dir) {
                Ok(_) => {
                    img_resize.create_mdpi(&all_path[0]);
                    img_resize.create_hdpi(&all_path[1]);
                    img_resize.create_xhdpi(&all_path[2]);
                    img_resize.create_xxhdpi(&all_path[3]);
                    img_resize.create_xxxdhpi(&all_path[4]);
                }
                Err(err) => match err.kind() {
                    ErrorKind::AlreadyExists => {
                        eprintln!("Directory already exists: {}", out_dir)
                    }
                    _ => panic!("Error creating directory: {}", err),
                },
            }
        }
    }
}

pub fn get_paths_by_target(outDir: &str, target: &str) -> Result<[String; 5], &'static str> {
    if target == "flutter" {
        return Ok([
            format!("{}/1.5x", outDir),
            format!("{}/2.0x", outDir),
            format!("{}/3.0x", outDir),
            format!("{}/4.0x", outDir),
            format!("{}", outDir),
        ]);
    }

    if target == "android" {
        return Ok([
            format!("{}/mipmap-mdpi", outDir),
            format!("{}/mipmap-hdpi", outDir),
            format!("{}/mipmap-xhdpi", outDir),
            format!("{}/mipmap-xxhdpi", outDir),
            format!("{}/mipmap-xxxhdpi", outDir),
        ]);
    }

    Err("Invalid target")
}
