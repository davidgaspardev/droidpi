extern crate image;

mod cli;
mod context;
mod platform;
mod resize;

use context::Context;

fn main() {
    println!("Welcome to droiDPI");

    let result = cli::get_arguments();
    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    let args = result.unwrap();
    let ctx = Context::new(args);

    let src = ctx.get_arg_src();
    let out_dir = ctx.get_arg_out_dir();
    let platform_name = ctx.get_arg_platform();
    let name = ctx.get_arg_name();
    let use_drawable = ctx.use_drawable();

    match image::open(src) {
        Ok(img) => {
            let img_resize = resize::Resize::new(img, name.to_string());

            match platform::PlatformFactory::get_platform_resize(
                platform_name,
                img_resize,
                use_drawable,
            ) {
                Ok(platform_target) => {
                    let result = platform_target.create_images(out_dir);

                    if let Err(msg) = result {
                        eprintln!("Error: {}", msg);
                        std::process::exit(1);
                    }
                    
                    println!("✓ Images created successfully!");
                }
                Err(msg) => {
                    eprintln!("Error: {}", msg);
                    std::process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Error: Failed to open image file '{}': {}", src, err);
            std::process::exit(1);
        }
    }
}
