extern crate image;

mod cli;
mod platform;
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

        let src = setting.get(cli::FLAG_SRC).unwrap();
        let out_dir = setting.get(cli::FLAG_OUTDIR).unwrap();
        let platform_name = setting.get(cli::FLAG_PLATFORM).unwrap();
        let name = setting.get(cli::FLAG_NAME).unwrap();

        if let Ok(img) = image::open(src) {
            let img_resize = resize::Resize::new(img, name.to_string());

            match platform::PlatformFactory::get_platform_resize(platform_name, img_resize) {
                Ok(platform_target) => {
                    let result = platform_target.create_images(out_dir);

                    if let Err(msg) = result {
                        eprint!("{}", msg);
                    }
                }
                Err(msg) => {
                    eprint!("{}", msg);
                }
            }
        }
    }
}
