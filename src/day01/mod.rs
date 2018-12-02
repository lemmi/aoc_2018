use std::io;
use std::collections::HashSet;


pub fn star1(lines: impl Iterator<Item = io::Result<String>>) -> super::StarResult {
    let s: isize = lines.map(|l| l.unwrap().parse::<isize>().unwrap()).sum();
    println!("{}", s);
    Ok(())
}

pub fn star2(lines: impl Iterator<Item = std::io::Result<String>>) -> super::StarResult {
    let diffs: Vec<isize> = lines.map(io::Result::unwrap).map(|x| x.parse::<isize>().unwrap()).collect();
    let freqs = diffs.iter().cycle().scan(0, |state, &x| { *state += x; Some(*state) });
    let mut seen = HashSet::new();

    for f in freqs {
        if !seen.insert(f) {
            println!("{}", f);
            break;
        }
    }
    Ok(())
}
