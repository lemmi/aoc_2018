use std::cmp::Ordering;
use std::ops::{Add, Neg};

use super::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd)]
struct Vector(isize, isize);

impl Vector {
    fn cw(&self) -> Vector {
        Vector(-self.1, self.0)
    }
    fn ccw(&self) -> Vector {
        Vector(self.1, -self.0)
    }
    fn char_or(&self, c: char) -> char {
        match (self.0, self.1) {
            (1, 0) => '>',
            (-1, 0) => '<',
            (0, 1) => 'v',
            (0, -1) => '^',
            _ => c,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector(-self.0, -self.1)
    }
}
impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1)
    }
}
impl Ord for Vector {
    fn cmp(&self, other: &Vector) -> Ordering {
        if self.1 == other.1 {
            self.0.cmp(&other.0)
        } else {
            self.1.cmp(&other.1)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Cart {
    pos: Vector,
    dir: Vector,
    intersections: usize,
}

impl Cart {
    fn mv(&self, track: char) -> Cart {
        let (nextdir, inc) = match (track, self.dir) {
            ('|', d) => {
                if d.0 != 0 {
                    unreachable!()
                } else {
                    (d, 0)
                }
            }
            ('-', d) => {
                if d.1 != 0 {
                    unreachable!()
                } else {
                    (d, 0)
                }
            }
            ('+', d) => match self.intersections % 3 {
                0 => (d.ccw(), 1),
                1 => (d, 1),
                2 => (d.cw(), 1),
                _ => unreachable!(),
            },
            ('/', Vector(x, y)) => (Vector(-y, -x), 0),
            ('\\', Vector(x, y)) => (Vector(y, x), 0),
            _ => unreachable!(),
        };
        Cart {
            pos: self.pos + nextdir,
            dir: nextdir,
            intersections: (self.intersections + inc) % 3,
        }
    }
    fn new(pos: Vector, dir: Vector) -> Cart {
        Cart {
            pos,
            dir,
            intersections: 0,
        }
    }
}

struct Tracks(Vec<Vec<char>>);

impl Tracks {
    fn parse(
        lines: impl Iterator<Item = std::io::Result<String>>,
    ) -> Result<(Tracks, Vec<Cart>), StarError> {
        let mut v = lines
            .map(map_error)
            .map(|r| r.map(|s| s.chars().collect::<Vec<char>>()))
            .collect::<Result<Vec<_>, _>>()?;

        let mut carts = Vec::new();
        for (y, l) in v.iter_mut().enumerate() {
            for (x, c) in l.iter_mut().enumerate() {
                let x = x as isize;
                let y = y as isize;
                *c = match c {
                    '<' => {
                        carts.push(Cart::new(Vector(x, y), Vector(-1, 0)));
                        '-'
                    }
                    '>' => {
                        carts.push(Cart::new(Vector(x, y), Vector(1, 0)));
                        '-'
                    }
                    'v' => {
                        carts.push(Cart::new(Vector(x, y), Vector(0, 1)));
                        '|'
                    }
                    '^' => {
                        carts.push(Cart::new(Vector(x, y), Vector(0, -1)));
                        '|'
                    }
                    ' ' | '|' | '-' | '+' | '/' | '\\' => *c,
                    c => unreachable!(format!("Got {}", c)),
                }
            }
        }

        Ok((Tracks(v), carts))
    }
    fn get(&self, v: Vector) -> char {
        self.0[v.1 as usize][v.0 as usize]
    }
    fn at_cart(&self, c: &Cart) -> char {
        self.get(c.pos)
    }
}

#[allow(dead_code)]
fn print(ts: &Tracks, cs: &[Cart]) {
    for (y, l) in ts.0.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            let x = x as isize;
            let y = y as isize;
            let c = if let Some(cart) = cs.iter().find(|c| c.pos == Vector(x, y)) {
                cart.dir.char_or(*c)
            } else {
                *c
            };
            print!("{}", c);
        }
        println!();
    }
}

fn collision(c: &Cart, cs: &[Cart]) -> Option<usize> {
    for (i, other) in cs.iter().enumerate() {
        if c.pos == other.pos {
            return Some(i);
        }
    }
    None
}

pub fn star2(lines: impl Iterator<Item = std::io::Result<String>>) -> StarResult {
    let (ts, mut cs) = Tracks::parse(lines)?;

    while cs.len() > 1 {
        cs.sort_by_key(|c| -c.pos);
        let mut moved = Vec::new();
        while let Some(c) = cs.pop() {
            let c = c.mv(ts.at_cart(&c));

            if let Some(other) = collision(&c, &moved) {
                moved.remove(other);
                continue;
            }
            if let Some(other) = collision(&c, &cs) {
                cs.remove(other);
                continue;
            }
            moved.push(c);
        }
        cs = moved;
    }
    println!("Last cart is at ({},{})", cs[0].pos.0, cs[0].pos.1);
    Ok(())
}

pub fn star1(lines: impl Iterator<Item = std::io::Result<String>>) -> StarResult {
    let (ts, mut cs) = Tracks::parse(lines)?;

    loop {
        cs.sort_by_key(|c| c.pos);
        for i in 0..cs.len() {
            let n = cs[i].mv(ts.at_cart(&cs[i]));

            for (k, other) in cs.iter().enumerate() {
                if i == k {
                    continue;
                }
                if n.pos == other.pos {
                    println!("Found collision at ({},{})", n.pos.0, n.pos.1);
                    return Ok(());
                }
            }

            cs[i] = n;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rotations() {
        assert_eq!(Vector(1, 0).cw(), Vector(0, 1));
        assert_eq!(Vector(0, 1).cw(), Vector(-1, 0));
        assert_eq!(Vector(-1, 0).cw(), Vector(0, -1));
        assert_eq!(Vector(0, -1).cw(), Vector(1, 0));

        assert_eq!(Vector(1, 0), Vector(0, 1).ccw());
        assert_eq!(Vector(0, 1), Vector(-1, 0).ccw());
        assert_eq!(Vector(-1, 0), Vector(0, -1).ccw());
        assert_eq!(Vector(0, -1), Vector(1, 0).ccw());
    }
    #[test]
    fn carts() {
        let c0 = Cart::new(Vector(0, 0), Vector(1, 0));
        let c1 = Cart::new(Vector(1, 0), Vector(1, 0));
        let c2 = Cart::new(Vector(0, 1), Vector(0, 1));
        let c3 = Cart::new(Vector(0, -1), Vector(0, -1));

        assert_eq!(c0.mv('-'), c1);
        assert_eq!(c0.mv('\\'), c2);
        assert_eq!(c0.mv('/'), c3);
    }

}
