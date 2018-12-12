use std::iter::repeat;
use std::ops::{Index, IndexMut};

fn power_level(x: isize, y: isize, serial: isize) -> isize {
    let rackid = x + 10;
    let p1 = rackid * y;
    let p2 = p1 + serial;
    let p3 = p2 * rackid;
    let hundrets = (p3 / 100) - ((p3 / 1000) * 10);
    hundrets - 5
}

struct Grid(Vec<isize>, usize);

impl Index<(isize, isize)> for Grid {
    type Output = isize;

    #[inline]
    fn index(&self, (x, y): (isize, isize)) -> &isize {
        if y < 1 || x < 1 {
            return &0;
        }
        let l = self.1;
        let y = (y - 1) as usize;
        let x = (x - 1) as usize;
        &self.0[y * l + x]
    }
}
impl IndexMut<(isize, isize)> for Grid {
    #[inline]
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        let l = self.1;
        let y = (y - 1) as usize;
        let x = (x - 1) as usize;
        &mut self.0[y * l + x]
    }
}
impl Default for Grid {
    fn default() -> Grid {
        Grid(vec![0isize; 300 * 300], 300)
    }
}
impl Grid {
    fn new(serial: isize) -> Grid {
        let mut g = Grid::default();
        for y in 1..=300 {
            for x in 1..=300 {
                g[(x, y)] =
                    power_level(x, y, serial) + g[(x - 1, y)] + g[(x, y - 1)] - g[(x - 1, y - 1)];
            }
        }
        g
    }
    fn box_power(&self, x: isize, y: isize, size: isize) -> isize {
        self[(x + size - 1, y + size - 1)]
            - self[(x + size - 1, y - 1)]
            - self[(x - 1, y + size - 1)]
            + self[(x - 1, y - 1)]
    }
    fn max_power(&self, box_size: isize) -> ((isize, isize), isize, isize) {
        (1..=301 - box_size)
            .flat_map(|y| (1..=301 - box_size).zip(repeat(y)))
            .map(|(x, y)| ((x, y), self.box_power(x, y, box_size), box_size))
            .max_by_key(|(_, pow, _)| *pow)
            .expect("Expect solution")
    }

    fn max_power_size(&self) -> ((isize, isize), isize, isize) {
        (1..=300)
            .map(|size| self.max_power(size))
            .max_by_key(|(_, pow, _)| *pow)
            .expect("Expect solution")
    }
}

pub fn star1(serial: isize) {
    let ((x, y), pow, _) = Grid::new(serial).max_power(3);
    println!("Box with largest power {} is at {},{}", pow, x, y);
}

pub fn star2(serial: isize) {
    let ((x, y), pow, size) = Grid::new(serial).max_power_size();
    println!("Box with largest power {} is at {},{},{}", pow, x, y, size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_power_levels() {
        assert_eq!(Grid::new(8).box_power(3, 5, 1), 4);
        assert_eq!(Grid::new(57).box_power(122, 79, 1), -5);
        assert_eq!(Grid::new(39).box_power(217, 196, 1), 0);
        assert_eq!(Grid::new(71).box_power(101, 153, 1), 4);
    }

    #[test]
    fn check_box_power() {
        assert_eq!(Grid::new(18).box_power(33, 45, 3), 29);
        assert_eq!(Grid::new(42).box_power(21, 61, 3), 30);
    }

    #[test]
    fn check_max_power() {
        assert_eq!(Grid::new(18).max_power(3), ((33, 45), 29, 3));
        assert_eq!(Grid::new(42).max_power(3), ((21, 61), 30, 3));
    }

    #[test]
    #[ignore]
    fn check_max_power_size() {
        assert_eq!(Grid::new(18).max_power_size(), ((90, 269), 113, 16));
        assert_eq!(Grid::new(42).max_power_size(), ((232, 251), 119, 12));
    }
}
