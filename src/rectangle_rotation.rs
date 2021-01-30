fn rectangle_rotation(a: i32, b: i32) -> i32 {
    debug_assert_eq!(a % 2, 0);
    debug_assert_eq!(b % 2, 0);

    let sqrt2 = 2.0f64.sqrt();
    let a = (a as f64) / sqrt2;
    let b = (b as f64) / sqrt2;

    let a = [a.ceil() as i32, a.floor() as i32];
    let b = [b.ceil() as i32, b.floor() as i32];

    let findmod = |dots: [i32; 2], y| dots.iter().copied().find(|&x| x % 2 == y).unwrap();

    findmod(a, 0) * findmod(b, 0) + findmod(a, 1) * findmod(b, 1)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(rectangle_rotation(6, 4), 23);
        assert_eq!(rectangle_rotation(30, 2), 65);
        assert_eq!(rectangle_rotation(8, 6), 49);
        assert_eq!(rectangle_rotation(16, 20), 333);
    }
}
