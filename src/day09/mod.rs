use super::*;
use std::collections::VecDeque;
use std::iter::FromIterator;

fn parse_input(s: std::io::Result<String>) -> Result<(usize, usize), StarError> {
    let s = s.map_err(StarError::from)?;
    let v = s
        .split_whitespace()
        .take(2)
        .map(|d| d.parse::<usize>().map_err(|e| e.into()))
        .collect::<Result<Vec<_>, StarError>>()?;

    if v.len() != 2 {
        Err(format!("Expected 2 inputs, got \"{:?}\"", v).into())
    } else {
        Ok((v[0], v[1]))
    }
}

#[derive(Debug)]
struct Playfield {
    circle: VecDeque<usize>,
}

impl Playfield {
    fn insert(&mut self, marble: usize) -> usize {
        if marble % 23 != 0 {
            self.rotate_cw(2);
            self.circle.push_front(marble);
            0
        } else {
            self.rotate_ccw(7);
            marble + self.circle.pop_front().unwrap()
        }
    }
    fn rotate_cw(&mut self, n: isize) {
        for _ in 0..n {
            let t = self.circle.pop_front().unwrap();
            self.circle.push_back(t);
        }
    }
    fn rotate_ccw(&mut self, n: isize) {
        for _ in 0..n {
            let t = self.circle.pop_back().unwrap();
            self.circle.push_front(t);
        }
    }
}
impl Default for Playfield {
    fn default() -> Self {
        Playfield {
            circle: VecDeque::from_iter(vec![0usize]),
        }
    }
}

fn play(p: usize, n: usize) -> usize {
    let mut playfield = Playfield::default();
    playfield.circle.reserve(n + 1);
    let mut scoreboard = Vec::new();
    scoreboard.resize(p, 0);
    for m in 1..=n {
        let score = playfield.insert(m);
        scoreboard[(m - 1) % p] += score;
    }
    *scoreboard.iter().max().expect("Expected high score")
}

pub fn star1(lines: impl Iterator<Item = std::io::Result<String>>) -> StarResult {
    for (p, n) in lines.map(parse_input).filter_map(|r| r.ok()) {
        let highscore = play(p, n);
        println!(
            "{} players, last marble is worth {} points: high score is {}",
            p, n, highscore
        );
    }

    Ok(())
}

pub fn star2(lines: impl Iterator<Item = std::io::Result<String>>) -> StarResult {
    for (p, n) in lines.map(parse_input).filter_map(|r| r.ok()) {
        let highscore = play(p, 100 * n);
        println!(
            "{} players, last marble is worth {} points: high score is {}",
            p,
            100 * n,
            highscore
        );
    }

    Ok(())
}
