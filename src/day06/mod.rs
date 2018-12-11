use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

use super::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn dist(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl FromStr for Point {
    type Err = StarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tok = s
            .split(',')
            .map(|s| s.trim_matches(char::is_whitespace))
            .map(|s| s.parse::<isize>().map_err(StarError::from))
            .collect::<Result<Vec<_>, Self::Err>>()?;

        if tok.len() != 2 {
            Err(format!("Wrong number of coorinates \"{:?}\"", tok).into())
        } else {
            Ok(Point::new(tok[0], tok[1]))
        }
    }
}

fn bounding_box(points: &[Point], fatten: isize) -> (Point, Point) {
    let minx = points
        .iter()
        .map(|p| p.x)
        .min()
        .expect("Error finding bounding box");
    let miny = points
        .iter()
        .map(|p| p.y)
        .min()
        .expect("Error finding bounding box");
    let maxx = points
        .iter()
        .map(|p| p.x)
        .max()
        .expect("Error finding bounding box");
    let maxy = points
        .iter()
        .map(|p| p.y)
        .max()
        .expect("Error finding bounding box");

    (
        Point::new(minx - fatten, miny - fatten),
        Point::new(maxx + fatten, maxy + fatten),
    )
}

fn far_points(points: &[Point]) -> HashSet<Point> {
    let (bbul, bblr) = bounding_box(points, 0);
    let bbw = bblr.x - bbul.x + 1;
    let bbh = bblr.y - bbul.y + 1;
    let ul = Point::new(bbul.x - bbw, bbul.y - bbh);
    let lr = Point::new(bblr.x + bbw, bblr.y + bbh);

    let mut ret = HashSet::new();

    let insert = |x, y, map: &mut HashSet<Point>| {
        if let Some(closest) = closest_to(points, Point::new(x, y)) {
            map.insert(closest);
        }
    };

    for x in ul.x..lr.x {
        insert(x, ul.y, &mut ret);
        insert(x, lr.y, &mut ret);
    }
    for y in ul.y..lr.y {
        insert(ul.x, y, &mut ret);
        insert(lr.x, y, &mut ret);
    }

    ret
}

fn closest_to(points: &[Point], p: Point) -> Option<Point> {
    let c = points
        .iter()
        .min_by_key(|ps| ps.dist(&p))
        .expect("Error finding closest Point");
    for ps in points {
        if ps == c {
            continue;
        }
        if ps.dist(&p) == c.dist(&p) {
            return None;
        }
    }
    Some(*c)
}

fn within_box(bb: (Point, Point)) -> impl Iterator<Item = (isize, isize)> {
    let (ul, lr) = bb;
    let w = (lr.x - ul.x + 1) as usize;

    (ul.x..=lr.x)
        .cycle()
        .zip((ul.y..=lr.y).flat_map(move |y| std::iter::repeat(y).take(w)))
}

fn coverage(points: &[Point]) -> HashMap<Point, usize> {
    let mut areas = HashMap::new();
    for (x, y) in within_box(bounding_box(points, 0)) {
        let p = Point::new(x, y);
        if let Some(closest) = closest_to(points, p) {
            //println!("{:?} is closest to {:?}", p, closest);
            let e = areas.entry(closest).or_insert(0);
            *e += 1;
        } else {
            //println!("{:?} has multiple closest points", p);
        }
    }
    areas
}

pub fn star1(lines: impl Iterator<Item = io::Result<String>>) -> StarResult {
    let points = lines
        .map(|l| l.map_err(StarError::from).and_then(|s| s.parse::<Point>()))
        .collect::<Result<Vec<Point>, StarError>>()?;

    for p in &points {
        println!("{:?}", p);
    }
    println!("Bounding box {:?}", bounding_box(&points, 0));

    let mut cover = coverage(&points);
    for (p, a) in &cover {
        println!("{:?}, {}", p, a);
    }

    let far = far_points(&points);
    println!("{:?}", far);

    for f in far {
        cover.remove(&f);
    }

    let (p, a) = cover
        .iter()
        .max_by_key(|(_, a)| *a)
        .expect("Expected a Point with maximum coverage");
    println!("Point with maximum area of {}: {:?}", a, p);

    Ok(())
}

pub fn star2(lines: impl Iterator<Item = std::io::Result<String>>) -> super::StarResult {
    let points = lines
        .map(|l| l.map_err(StarError::from).and_then(|s| s.parse::<Point>()))
        .collect::<Result<Vec<Point>, StarError>>()?;

    let mut area = 0;
    for (x, y) in within_box(bounding_box(&points, 10000 / points.len() as isize)) {
        let p = Point::new(x, y);
        let dist: isize = points.iter().map(|ps| ps.dist(&p)).sum();
        if dist < 10000 {
            area += 1;
        }
    }
    println!(
        "Size of the area of locations less then 10000 away is {}",
        area
    );

    Ok(())
}
