use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::time::Instant;

extern crate chrono;
extern crate itertools;

use chrono::{DateTime, Datelike, Local};

mod day01;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

#[derive(Debug)]
pub enum StarError {
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    IoError(std::io::Error),
    StringError(String),
}

impl Display for StarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::num::ParseIntError> for StarError {
    fn from(err: std::num::ParseIntError) -> StarError {
        StarError::ParseIntError(err)
    }
}
impl From<std::num::ParseFloatError> for StarError {
    fn from(err: std::num::ParseFloatError) -> StarError {
        StarError::ParseFloatError(err)
    }
}
impl From<std::io::Error> for StarError {
    fn from(err: std::io::Error) -> StarError {
        StarError::IoError(err)
    }
}
impl From<String> for StarError {
    fn from(err: String) -> StarError {
        StarError::StringError(err)
    }
}
impl<'a> From<&'a str> for StarError {
    fn from(err: &str) -> StarError {
        StarError::StringError(err.to_string())
    }
}

impl Error for StarError {}

pub type StarResult = Result<(), StarError>;
fn map_error(r: io::Result<String>) -> Result<String, StarError> {
    r.map_err(|e| e.into())
}

fn main() -> Result<(), StarError> {
    let local: DateTime<Local> = Local::now();
    println!("Running day {}", local.day());
    let start = Instant::now();
    match local.day() {
        1 => {
            day01::star1(solve("day01/input")?)?;
            day01::star2(solve("day01/input")?)?;
        }
        3 => {
            day03::star1(solve("day03/input")?)?;
            day03::star2(solve("day03/input")?)?;
        }
        4 => {
            day04::star1(solve("day04/input")?)?;
            day04::star2(solve("day04/input")?)?;
        }
        5 => {
            day05::star1(solve("day05/input")?)?;
            day05::star2(solve("day05/input")?)?;
        }
        6 => {
            day06::star1(solve("day06/input")?)?;
            day06::star2(solve("day06/input")?)?;
        }
        7 => {
            day07::star1(solve("day07/input")?)?;
            day07::star2(solve("day07/input")?)?;
        }
        8 => {
            day08::star1(solve("day08/input")?)?;
            day08::star2(solve("day08/input")?)?;
        }
        9 => {
            day09::star1(solve("day09/input")?)?;
            day09::star2(solve("day09/input")?)?;
        }
        10 => {
            day10::star12(solve("day10/input")?)?;
        }
        11 => {
            day11::star1(9221);
            day11::star2(9221);
        }
        12 => {
            day12::star1(solve("day12/input")?)?;
            day12::star2(solve("day12/input")?)?;
        }
        13 => {
            day13::star1(solve("day13/input")?)?;
            day13::star2(solve("day13/input")?)?;
        }
        14 => {
            day14::star1(702_831)?;
            day14::star2(702_831)?;
        }
        _ => (),
    }
    println!("Took {:?}", start.elapsed());
    Ok(())
}

fn solve(input: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(input)?;
    let br = BufReader::new(file);
    let iter = br.lines();
    Ok(iter)
}
