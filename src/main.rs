use chrono::Datelike;
use clap::{App, Arg};
use errors::{ACResult, Error};

mod advent2018;
mod errors;
mod utils;

#[macro_use]
extern crate clap;

extern crate chrono;

fn main() -> ACResult<()> {
    let matches = App::new("Advent solver")
        .version("0.1.0")
        .author("AsamK <asamk@gmx.de>")
        .about("Solves puzzles")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .value_name("YEAR")
                .help("Choose the advent year (default: current year)")
                .requires("day")
                .takes_value(true),
        ).arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("Choose the advent day (default: current day)")
                .takes_value(true),
        ).arg(
            Arg::with_name("level")
                .short("l")
                .long("level")
                .value_name("LEVEL")
                .required(true)
                .help("Choose the advent year")
                .takes_value(true),
        ).arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Make output more verbose"),
        ).get_matches();

    let hour = 3600;
    // Advent website is in TimeZone EST/UTC-5
    let timezone = chrono::FixedOffset::west(5 * hour);

    let year = if matches.is_present("year") {
        value_t!(matches.value_of("year"), i32).unwrap_or_else(|e| e.exit())
    } else {
        chrono::Local::now().with_timezone(&timezone).year()
    };

    let day = if matches.is_present("day") {
        value_t!(matches.value_of("day"), u8).unwrap_or_else(|e| e.exit())
    } else {
        chrono::Local::now().with_timezone(&timezone).day() as u8
    };

    let level = value_t!(matches.value_of("level"), u8).unwrap_or_else(|e| e.exit());

    let verbose = matches.is_present("verbose");

    if verbose {
        eprintln!("Solving puzzle for {} day {} level {}", year, day, level);
    }

    let stdin = std::io::stdin();
    let data = stdin.lock();

    let result = match year {
        2018 => advent2018::get_result(data, day, level),
        _ => Err(Error::new(format!("Year {} is not implemented", year))),
    }?;

    println!("{}", result);

    Ok(())
}
