use clap::Parser;

mod days;
use days::*;

#[derive(Parser)]
struct Cli {
    day: u8
}

fn main() {
    let cli = Cli::parse();
    match cli.day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        _ => println!("Invalid day")
    }
}
