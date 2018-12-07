use chrono::Datelike;
use clap::{App, Arg};
use errors::{ACResult, Error};
use std::io::Read;

mod advent2017;
mod advent2018;
mod errors;
mod utils;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate serde_derive;

extern crate toml;

extern crate chrono;

extern crate xdg;

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
                .help("Choose the advent year [default: current year]")
                .requires("day")
                .takes_value(true),
        ).arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("Choose the advent day [default: current day]")
                .takes_value(true),
        ).arg(
            Arg::with_name("level")
                .short("l")
                .long("level")
                .value_name("LEVEL")
                .default_value("1")
                .help("Choose the level")
                .takes_value(true),
        ).arg(
            Arg::with_name("input-file")
                .short("f")
                .long("input-file")
                .value_name("INPUT_FILE")
                .help("Specify a file to use as puzzle input. [default: Download user specific input from adventofcode.com]")
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

    let xdg_dirs = xdg::BaseDirectories::with_prefix("advent_of_code_solver").unwrap();

    let config: Option<Config> = if let Some(config_path) = xdg_dirs.find_config_file("config.toml")
    {
        let mut logo_file = std::fs::File::open(config_path)
            .map_err(|_| Error::new_str("Failed to load config file."))?;
        let mut config_string = String::new();
        logo_file
            .read_to_string(&mut config_string)
            .map_err(|_| Error::new_str("Failed to load config file."))?;
        println!("{}", &config_string);
        Some(
            toml::from_str(&config_string)
                .map_err(|_| Error::new_str("Failed to load config file."))?,
        )
    } else {
        None
    };

    let input_file = if matches.is_present("input-file") {
        std::fs::File::open(matches.value_of_os("input-file").unwrap())
            .map_err(|_| Error::new_str("Failed to load input file."))?
    } else {
        let input_file_name = format!("input/{}/{}", year, day);
        if let Some(input_file_path) = xdg_dirs.find_data_file(&input_file_name) {
            std::fs::File::open(input_file_path)
                .map_err(|_| Error::new_str("Failed to load input file."))?
        } else {
            // Download
            let session = config
                .ok_or(Error::new_str("No session cookie to download input"))?
                .session_token
                .ok_or(Error::new_str("No session cookie to download input"))?;
            let request_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
            let mut body = reqwest::Client::new()
                .request(reqwest::Method::GET, &request_url)
                .header(reqwest::header::COOKIE, format!("session={}", session))
                .send()
                .map_err(|_| Error::new_str("Failed to download input file."))?;
            if !body.status().is_success() {
                return Err(Error::new(format!(
                    "Failed to download input file: {}",
                    body.status()
                )));
            }

            if let Ok(input_file_path) = xdg_dirs.place_data_file(&input_file_name) {
                println!("{:?}", &input_file_path);
                {
                    let mut input_file = std::fs::File::create(&input_file_path)
                        .map_err(|_| Error::new_str("Failed to store downloaded input file."))?;
                    body.copy_to(&mut input_file)
                        .map_err(|_| Error::new_str("Failed to store downloaded input file."))?;
                }

                std::fs::File::open(&input_file_path)
                    .map_err(|_| Error::new_str("Failed to load input file."))?
            } else {
                Err(Error::new_str("Failed to store download file"))?
            }
        }
    };
    let data = std::io::BufReader::new(input_file);

    let result = match year {
        2017 => advent2017::get_result(data, day, level),
        2018 => advent2018::get_result(data, day, level),
        _ => Err(Error::new(format!("Year {} is not implemented", year))),
    }?;

    println!("{}", result);

    Ok(())
}

#[derive(Deserialize)]
struct Config {
    session_token: Option<String>,
}
