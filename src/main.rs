extern crate image;

use image::GenericImageView;

mod cli;

fn main() {
    println!("Welcome to droiDPI");

    let result = cli::get_arguments();

    match result {
        Err(error) => println!("{}", error),
        Ok(setting) => {
            println!("setting: {:?}", setting);
            println!("setting[IMAGE_PATH]: {}", setting[cli::IMAGE_PATH]);
            println!("setting[FLUTTER_PATH]: {}", setting[cli::FLUTTER_PATH]);

            let img = image::open(format!("{}", setting[cli::IMAGE_PATH])).unwrap();

            println!{"Dimensions: {:?}", img.dimensions()};
        }
    }
}