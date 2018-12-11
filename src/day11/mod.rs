use super::*;

fn power_level(x: isize, y: isize, serial: isize) -> isize {
    let rackid = x + 10;
    let p1 = rackid * y;
    let p2 = p1 + serial;
    let p3 = p2 * rackid;
    let hundrets = (p3 / 100) - ((p3 / 1000) * 10);
    hundrets - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_power_levels() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
