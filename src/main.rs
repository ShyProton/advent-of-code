use std::env::{self};

mod year2021;
mod year2022;
mod year2023;

fn main() {
    let args: Vec<String> = env::args().collect();

    let module = args.get(1).map_or_else(
        || {
            println!("No year name specified, defaulting to year2022.");
            "year2022"
        },
        |module| module,
    );

    match module {
        "year2022" => year2022::select_day(&args),
        "year2023" => year2023::select_day(&args),
        _ => println!("Invalid module name."),
    }
}
