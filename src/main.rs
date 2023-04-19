extern crate image;

mod cli;
mod resize;

fn main() {
    println!("Welcome to droiDPI");

    let result = cli::get_arguments();

    match result {
        Err(error) => println!("{}", error),
        Ok(setting) => {
            println!("setting: {:?}", setting);
            println!("setting[IMAGE_PATH]: {}", setting[cli::IMAGE_PATH]);
            println!("setting[FLUTTER_PATH]: {}", setting[cli::FLUTTER_PATH]);
            println!("setting[IMAGE_NAME]: {}", setting[cli::IMAGE_NAME]);

            let image_path = &setting[cli::IMAGE_PATH];
            let flutter_path = &setting[cli::FLUTTER_PATH];
            let image_name = &setting[cli::IMAGE_NAME];

            match image::open(image_path) {
                Err(err) => println!("{}", err),
                Ok(img ) => {
                    /* Resize image to flutter suport */
                    let r = resize::Resize::new(img, image_name.to_string());
                    r.create_xxhdpi();
                    r.create_xhdpi();
                    r.create_hdpi();
                    r.create_mdpi();
                }
            }
        }
    }
}