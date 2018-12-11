use std::iter::FromIterator;
use std::ops::{Add, AddAssign, Neg, Sub};
use std::str::FromStr;

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Vector {
    x: isize,
    y: isize,
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = *self + other;
    }
}
impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Self::Output {
        self + (-other)
    }
}
impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vector {
    fn new(x: isize, y: isize) -> Vector {
        Vector { x, y }
    }
    fn min(self, other: Vector) -> Vector {
        Vector::new(self.x.min(other.x), self.y.min(other.y))
    }
    fn max(self, other: Vector) -> Vector {
        Vector::new(self.x.max(other.x), self.y.max(other.y))
    }
    fn min_value() -> Vector {
        Vector::new(isize::min_value(), isize::min_value())
    }
    fn max_value() -> Vector {
        Vector::new(isize::max_value(), isize::max_value())
    }
    fn bb(vs: impl Iterator<Item = Vector>) -> (Vector, Vector) {
        vs.fold(
            (Vector::max_value(), Vector::min_value()),
            |(min, max), v| (v.min(min), v.max(max)),
        )
    }
    fn area((min, max): (Vector, Vector)) -> usize {
        let a = max - min;
        (a.x * a.y) as usize
    }
}

impl IntoIterator for Vector {
    type Item = isize;
    type IntoIter = ::std::vec::IntoIter<isize>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y].into_iter()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Light {
    pos: Vector,
    v: Vector,
}

impl FromStr for Light {
    type Err = StarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tok: Vec<_> = s
            .split(|c| c == '<' || c == '>' || c == ',')
            .map(str::trim)
            .collect();

        if tok.len() != 7 {
            return Err(format!("Expected 6 tokens, got {}: {:?}", tok.len(), tok).into());
        }

        let px = tok[1].parse::<isize>()?;
        let py = tok[2].parse::<isize>()?;
        let vx = tok[4].parse::<isize>()?;
        let vy = tok[5].parse::<isize>()?;

        Ok(Light {
            pos: Vector::new(px, py),
            v: Vector::new(vx, vy),
        })
    }
}

impl Light {
    fn mv(self) -> Light {
        Light {
            pos: self.pos + self.v,
            v: self.v,
        }
    }
    fn bb(ls: impl Iterator<Item = Light>) -> (Vector, Vector) {
        Vector::bb(ls.map(|l| l.pos))
    }
}

fn move_all(v: &[Light]) -> Vec<Light> {
    v.iter().map(|l| l.mv()).collect()
}

fn print_field(v: &[Light]) {
    let (min, max) = Light::bb(v.iter().cloned());

    let w = max.x - min.x + 1;
    let h = max.y - min.y + 1;
    let mut f = vec!['.'; (w * h) as usize];

    for l in v.iter().map(|l| l.pos) {
        let dist = l - min;
        f[(dist.y * w + dist.x) as usize] = '#';
    }

    for l in f.chunks(w as usize) {
        println!("{}", String::from_iter(l));
    }
}

fn parse_input(
    lines: impl Iterator<Item = std::io::Result<String>>,
) -> Result<Vec<Light>, StarError> {
    lines
        .map(|r| r.map_err(|e| e.into()).and_then(|s| s.parse::<Light>()))
        .collect()
}

pub fn star12(lines: impl Iterator<Item = std::io::Result<String>>) -> StarResult {
    let mut lights = parse_input(lines)?;
    let mut bb = Light::bb(lights.iter().cloned());
    let mut a = Vector::area(bb);

    for step in 1.. {
        let next_lights = move_all(&lights);
        let next_bb = Light::bb(next_lights.iter().cloned());

        let next_a = Vector::area(next_bb);

        if next_a > a {
            println!("Seconds {}, size {}: ({:?})", step - 1, a, bb.1 - bb.0);
            print_field(&lights);
            println!();

            break;
        }

        bb = next_bb;
        lights = next_lights;
        a = next_a;
    }

    Ok(())
}
