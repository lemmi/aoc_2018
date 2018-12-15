use std::io;
use std::vec::Vec;

fn count_same_first(v: &[char]) -> (usize) {
    if let Some(&first) = v.last() {
        v.iter()
            .rev()
            .position(|&c| c != first)
            .unwrap_or_else(|| v.len())
    } else {
        0
    }
}
fn item_counts(v: &str) -> Vec<usize> {
    let mut chars: Vec<char> = v.chars().collect();
    chars.sort();
    let mut ret: Vec<usize> = Vec::new();
    while !chars.is_empty() {
        let n = count_same_first(&chars);
        let drop = chars.len() - n;
        chars.truncate(drop);
        ret.push(n);
    }
    ret.sort();
    ret.dedup();
    ret
}

pub fn star1(lines: impl Iterator<Item = io::Result<String>>) -> super::StarResult {
    let mut twos = 0;
    let mut threes = 0;
    for counts in lines.filter_map(|l| l.ok().map(|s| item_counts(&s))) {
        if counts.contains(&2usize) {
            twos += 1;
        }
        if counts.contains(&3usize) {
            threes += 1;
        }
    }
    println!("{} * {} = {}", twos, threes, twos * threes);
    Ok(())
}

pub fn star2(lines: impl Iterator<Item = io::Result<String>>) -> super::StarResult {
    let ids: Vec<Vec<char>> = lines
        .map(|l| l.map(|s| s.chars().collect()))
        .collect::<io::Result<Vec<_>>>()?;

    for (i, left) in ids[..ids.len() - 1].iter().enumerate() {
        for right in ids[i + 1..].iter() {
            let mut same: Vec<char> = Vec::new();
            for (l, r) in left.iter().zip(right) {
                if l == r {
                    same.push(*l);
                }
            }
            if same.len() == left.len() - 1 {
                let s: String = same.iter().collect();
                println!("{}", s);
                break;
            }
        }
    }

    Ok(())
}
