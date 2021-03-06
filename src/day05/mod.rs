use std::collections::BTreeSet;
use std::io;
use std::iter::repeat;
use std::iter::FromIterator;

use super::*;

fn can_react(a: char, b: char) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase() && a != b
}

fn react(v: &[char]) -> Vec<char> {
    let mut ret = Vec::new();
    if v.len() < 2 {
        return ret;
    }

    let mut last = v.first().cloned();
    for c in v[1..].iter() {
        match last {
            Some(l) => {
                if !can_react(l, *c) {
                    ret.push(l);
                    last = Some(*c);
                } else {
                    last = None;
                }
            }
            None => {
                last = Some(*c);
            }
        }
    }
    if let Some(l) = last {
        ret.push(l);
    }
    ret
}

fn react_full(v: &[char]) -> Vec<char> {
    let mut poly = v.to_owned();
    loop {
        let l = poly.len();
        poly = react(&poly);
        if l == poly.len() {
            break;
        }
    }
    poly
}

fn remove_and_react_full(v: &[char], a: char) -> Vec<char> {
    let v = v
        .iter()
        .filter(|c| c.to_ascii_lowercase() != a)
        .cloned()
        .collect::<Vec<_>>();
    react_full(&v)
}

pub fn star1(lines: impl Iterator<Item = io::Result<String>>) -> StarResult {
    let poly = lines
        .take(1)
        .next()
        .ok_or("No input")??
        .chars()
        .collect::<Vec<_>>();
    let reacted = react_full(&poly);
    println!("Reduced polymer is {} long", reacted.len());
    Ok(())
}

pub fn star2(lines: impl Iterator<Item = std::io::Result<String>>) -> super::StarResult {
    let poly = lines
        .take(1)
        .next()
        .ok_or("No input")??
        .chars()
        .collect::<Vec<_>>();
    let alphabet = BTreeSet::from_iter(poly.iter().map(|c| c.to_ascii_lowercase()));
    println!("Alphabet: {:?}", alphabet);

    let (a, p) = alphabet
        .iter()
        .zip(repeat(poly))
        .map(|(a, p)| (a, remove_and_react_full(&p, *a)))
        .min_by_key(|(_, p)| p.len())
        .ok_or("Unable to find best polymer")?;

    println!(
        "Best polymer found by removing '{}' with length {}: {}",
        a,
        p.len(),
        String::from_iter(p)
    );

    Ok(())
}
