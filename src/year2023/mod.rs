mod day1;
mod day2;

pub fn select_day(args: &[String]) {
    let module = args.get(2).map_or_else(
        || {
            println!("No day name specified, defaulting to day1.");
            "day1"
        },
        |module| module,
    );

    match module {
        "day1" => day1::main(),
        "day2" => day2::main(),
        _ => println!("Invalid module name."),
    }
}
