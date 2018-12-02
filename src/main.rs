use std::fmt::{self, Display, Debug};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Lines};

mod day01;

pub type StarResult = Result<(), Box<Error>>;

#[derive(Debug)]
pub struct StarError<T: Display + Debug> {
    desc: T,
}

impl<T: Display + Debug> Display for StarError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl<T: Display + Debug> Error for StarError<T> {
}

fn main() -> Result<(), Box<Error>> {
    day01::star1(solve("day01/input")?)?;
    day01::star2(solve("day01/input")?)?;
    Ok(())
}

fn solve(input: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(input)?;
    let br = BufReader::new(file);
    let iter = br.lines();
    Ok(iter)
}
