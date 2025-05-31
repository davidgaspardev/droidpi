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
        let name = &setting[cli::FLAG_NAME];

        if let Ok(img) = image::open(src) {
            let img_resize = resize::Resize::new(img, name.to_string());

            let platform_target =
                platform::PlatformFactory::get_platform_resize(platform_name, img_resize);

            if let Err(msg) = platform_target.create_images(out_dir) {
                eprintln!("{}", msg)
            }
        }
    }
}
