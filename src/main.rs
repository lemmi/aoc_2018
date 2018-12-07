use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

extern crate chrono;
extern crate itertools;

mod day01;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

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

fn main() -> Result<(), StarError> {
    day01::star1(solve("day01/input")?)?;
    day01::star2(solve("day01/input")?)?;
    day03::star1(solve("day03/input")?)?;
    day03::star2(solve("day03/input")?)?;
    day04::star1(solve("day04/input")?)?;
    day04::star2(solve("day04/input")?)?;
    day05::star1(solve("day05/input")?)?;
    day05::star2(solve("day05/input")?)?;
    day06::star1(solve("day06/input")?)?;
    day06::star2(solve("day06/input")?)?;
    day07::star1(solve("day07/input")?)?;
    day07::star2(solve("day07/input")?)?;
    Ok(())
}

fn solve(input: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(input)?;
    let br = BufReader::new(file);
    let iter = br.lines();
    Ok(iter)
}
