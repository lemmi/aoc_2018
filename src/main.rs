use std::io::{self, BufRead};

mod day01;

fn main() ->  Result<(), std::io::Error> {
    {
        let stdin = io::stdin();
        day01::solve(stdin.lock().lines())?;
    }
    Ok(())
}
