use std::iter::FromIterator;

use super::*;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
struct Game {
    state: Vec<bool>,
    rules: [bool; 32],
    offset: isize,
}

impl Game {
    fn parse_pot(c: char) -> bool {
        match c {
            '.' => false,
            '#' => true,
            c => panic!("Unkown character {}", c),
        }
    }
    fn parse_rule(s: &str) -> (usize, bool) {
        let cs = s.chars().collect::<Vec<char>>();
        let from = cs[..5]
            .iter()
            .cloned()
            .map(Self::parse_pot)
            .fold(0, |sum, c| (sum << 1) + if c { 1 } else { 0 });
        let to = Self::parse_pot(cs[9]);
        (from, to)
    }
    fn parse_state(s: &str) -> Vec<bool> {
        s.trim_start_matches("initial state: ")
            .chars()
            .map(Self::parse_pot)
            .collect::<Vec<bool>>()
    }
    fn pot_to_char(p: bool) -> char {
        if p {
            '#'
        } else {
            '.'
        }
    }
    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Result<Game, StarError> {
        let mut g = Game::default();

        let mut ls = lines.map(|r| r.map_err(StarError::from));
        g.state = Game::parse_state(&ls.next().ok_or("Unexpected end of input")??);
        ls.next().ok_or("Unexpected end of input")??;
        for l in ls {
            let (from, to) = Game::parse_rule(&l?);
            g.rules[from] = to;
        }
        Ok(g)
    }
    fn next_gen(&self) -> Game {
        let mut t = Vec::new();
        t.extend(vec![false; 5]);
        t.extend(self.state.iter());
        t.extend(vec![false; 5]);

        let state: Vec<bool> = t
            .windows(5)
            .map(|w| {
                w.iter()
                    .fold(0, |sum, c| (sum << 1) + if *c { 1 } else { 0 })
            })
            .map(|inp| self.rules[inp])
            .collect();

        let l = state.iter().position(|t| *t).unwrap_or(0);
        let r = state
            .iter()
            .rposition(|t| *t)
            .unwrap_or_else(|| state.len() - 1);

        let state = state[l..=r].to_owned();

        Game {
            state,
            rules: self.rules,
            offset: self.offset - 3 + l as isize,
        }
    }
    fn count(&self) -> isize {
        self.state
            .iter()
            .zip(self.offset..)
            .map(|(p, i)| if *p { i } else { 0 })
            .sum()
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            String::from_iter(self.state.iter().cloned().map(Game::pot_to_char))
        )
    }
}

pub fn star1(lines: impl Iterator<Item = std::io::Result<String>>) -> StarResult {
    let mut g = Game::parse(lines)?;

    for _ in 1..=20 {
        g = g.next_gen();
    }

    println!("Count: {:13} {}", g.count(), g);
    Ok(())
}

pub fn star2(lines: impl Iterator<Item = std::io::Result<String>>) -> StarResult {
    let mut g = Game::parse(lines)?;
    let mut m = std::collections::HashMap::new();

    for t in 1..=50_000_000_000isize {
        let (next, diff) = m
            .entry(g.state.clone())
            .or_insert_with(|| {
                let next = g.next_gen();
                let diff = next.offset - g.offset;
                (next.state, diff)
            })
            .clone();

        if g.state == next {
            g.offset += (50_000_000_000isize - t + 1) * diff;
            break;
        }

        g.state = next;
        g.offset += diff;
    }

    println!("Count: {:13} {}", g.count(), g);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_rule() {
        assert_eq!(Game::parse_rule("...## => #"), (0b00011, true));
        assert_eq!(Game::parse_rule("..#.. => #"), (0b00100, true));
        assert_eq!(Game::parse_rule(".#... => #"), (0b01000, true));
        assert_eq!(Game::parse_rule(".#.#. => #"), (0b01010, true));
        assert_eq!(Game::parse_rule(".#.## => #"), (0b01011, true));
        assert_eq!(Game::parse_rule(".##.. => #"), (0b01100, true));
        assert_eq!(Game::parse_rule(".#### => #"), (0b01111, true));
        assert_eq!(Game::parse_rule("#.#.# => #"), (0b10101, true));
        assert_eq!(Game::parse_rule("#.### => #"), (0b10111, true));
        assert_eq!(Game::parse_rule("##.#. => #"), (0b11010, true));
        assert_eq!(Game::parse_rule("##.## => #"), (0b11011, true));
        assert_eq!(Game::parse_rule("###.. => #"), (0b11100, true));
        assert_eq!(Game::parse_rule("###.# => #"), (0b11101, true));
        assert_eq!(Game::parse_rule("####. => #"), (0b11110, true));
    }

    #[test]
    fn test_example() {
        let file = File::open("src/day12/test").unwrap();
        let br = BufReader::new(file);
        let iter = br.lines();
        star1(iter).unwrap();
    }
}
