extern crate image;

mod cli;

fn main() {
    println!("Welcome to droiDPI");

    let result = cli::get_arguments();

    match result {
        Err(error) => println!("{}", error),
        _ => (),
    }
}