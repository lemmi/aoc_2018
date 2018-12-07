use std::cmp;
use std::io;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Rec {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    id: Option<usize>,
}

impl std::str::FromStr for Rec {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s
            .split(|c: char| {
                c.is_whitespace() || c == '#' || c == '@' || c == ',' || c == ':' || c == 'x'
            })
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, Self::Err>>()?;
        Ok(Rec::new(
            tokens[1],
            tokens[2],
            tokens[3],
            tokens[4],
            Some(tokens[0]),
        ))
    }
}

impl Rec {
    fn new(x: usize, y: usize, w: usize, h: usize, id: Option<usize>) -> Rec {
        Rec {
            x: x,
            y: y,
            w: w,
            h: h,
            id: id,
        }
    }
    fn area(&self) -> usize {
        self.w * self.h
    }

    fn intersect(&self, other: &Rec) -> Option<Rec> {
        let left = cmp::max(self.x, other.x) as isize;
        let right = cmp::min(self.x + self.w, other.x + other.w) as isize;
        let upper = cmp::max(self.y, other.y) as isize;
        let lower = cmp::min(self.y + self.h, other.y + other.h) as isize;

        let w = right - left;
        let h = lower - upper;

        if w < 1 || h < 1 {
            None
        } else {
            Some(Rec::new(
                left as usize,
                upper as usize,
                w as usize,
                h as usize,
                None,
            ))
        }
    }
    fn bounding_box(&self, other: &Rec) -> Rec {
        let left = cmp::min(self.x, other.x);
        let right = cmp::max(self.x + self.w, other.x + other.w);
        let upper = cmp::min(self.y, other.y);
        let lower = cmp::max(self.y + self.h, other.y + other.h);

        Rec::new(left, upper, right - left, lower - upper, None)
    }
}

fn check_not_intersect(v: &Vec<Rec>) -> Option<Rec> {
    'outer: for (i, a) in v.iter().enumerate() {
        for (j, b) in v.iter().enumerate() {
            if i == j {
                continue;
            }
            if let Some(_) = a.intersect(b) {
                continue 'outer;
            }
        }
        return Some(*a);
    }
    None
}

fn bounding_box(v: &Vec<Rec>) -> Rec {
    v.iter().fold(v[0], |acc, r| acc.bounding_box(r))
}

fn mark(v: &mut Vec<usize>, bb: &Rec, r: &Rec) {
    for y in r.y..r.y + r.h {
        for x in r.x..r.x + r.w {
            v[((y - bb.y) * bb.w + x - bb.x) as usize] += 1;
            //for l in v.chunks(bb.w as usize) {
            //    println!("{:?}",l);
            //}
            //println!("{:?} {:?} {} {}", bb, r, y, x);
        }
    }
}
pub fn star1(lines: impl Iterator<Item = io::Result<String>>) -> super::StarResult {
    let boxes = lines
        .map(|l| l.unwrap())
        .map(|s| s.parse::<Rec>())
        .collect::<Result<Vec<Rec>, _>>()?;
    let bb = bounding_box(&boxes);

    let mut bitmap: Vec<usize> = Vec::new();
    bitmap.resize(bb.area() as usize, 0);

    for r in &boxes {
        //for l in bitmap.chunks(bb.w as usize) {
        //    println!("{:?}",l)
        //}
        mark(&mut bitmap, &bb, &r)
    }

    let cover: usize = bitmap.iter().map(|t| if *t > 1 { 1 } else { 0 }).sum();
    let claims: usize = bitmap.iter().sum();
    let area: usize = boxes.iter().map(|r| r.area()).sum();
    println!(
        "{} squares are covered, with over {} claims ({})",
        cover, claims, area
    );

    Ok(())
}

pub fn star2(lines: impl Iterator<Item = std::io::Result<String>>) -> super::StarResult {
    let boxes = lines
        .map(|l| l.unwrap())
        .map(|s| s.parse::<Rec>())
        .collect::<Result<Vec<Rec>, _>>()?;
    match check_not_intersect(&boxes) {
        Some(r) => println!("Found non-overlapping claim: {:?}", r),
        None => println!("Couldn't find a non-overlapping claim!"),
    }
    Ok(())
}
