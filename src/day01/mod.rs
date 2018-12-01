use std::io;

pub fn solve(lines: impl Iterator<Item = io::Result<String>>) -> Result<(), std::io::Error> {
    for l in lines {
        println!("{}", l?);
    }
    Ok(())
}
