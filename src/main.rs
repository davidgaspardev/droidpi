extern crate image;

mod cli;
mod context;
mod platform;
mod resize;

use context::Context;
use context::Mode;

fn main() {
    println!("Welcome to droiDPI");

    let result = cli::get_arguments();
    if let Err(err) = result {
        eprintln!("{}", err);
        return;
    }

    if let Ok(args) = result {
        let ctx = Context::new(args);

        match ctx.mode {
            Mode::RunMainLogic => {
                println!("arguments: {:?}", ctx.args);

                let src = ctx.get_arg_src();
                let out_dir = ctx.get_arg_out_dir();
                let platform_name = ctx.get_arg_platform();
                let name = ctx.get_arg_name();

                if let Ok(img) = image::open(src) {
                    let img_resize = resize::Resize::new(img, name.to_string());

                    match platform::PlatformFactory::get_platform_resize(platform_name, img_resize)
                    {
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
            Mode::ShowVersion => {
                println!("{} v{}", ctx.name, ctx.version);
            }
        }
    }
}
