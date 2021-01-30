fn zeroes(base: i32, number: i32) -> i32 {
    dbg!(base, number);

    let factors = (2..base)
        .rev()
        .filter(|i| base % i == 0)
        .collect::<Vec<_>>();

    let result = (2..=number).fold((0, 1i64), |(mut zeros, mut acc), mut v| {
        while v % base == 0 {
            v /= base;
            zeros += 1;
        }

        for factor in factors.iter().copied() {
            while v % factor == 0 {
                v /= factor;
                acc *= factor as i64;
            }
        }

        let base = base as i64;
        while acc % base == 0 {
            acc /= base;
            zeros += 1;
        }

        (zeros, (acc))
    });
    dbg!(result);

    result.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_cases() {
        assert_eq!(zeroes(100, 50), 6);
        assert_eq!(zeroes(10, 1000000), 0);
        assert_eq!(zeroes(170, 100), 0);

        assert_eq!(zeroes(10, 10), 2);
        assert_eq!(zeroes(16, 16), 3);

        assert_eq!(zeroes(40, 10), 2);
        assert_eq!(zeroes(17, 16), 0);
        assert_eq!(zeroes(12, 26), 0);
        assert_eq!(zeroes(2, 524288), 524287);
        assert_eq!(zeroes(128, 7442), 0);
        assert_eq!(zeroes(256, 1000), 0);
        assert_eq!(zeroes(17, 100), 0);
    }
}
