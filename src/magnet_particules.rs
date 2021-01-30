fn doubles(maxk: i32, maxn: i32) -> f64 {
    fn v(k: i32, n: i32) -> f64 {
        let a = 1. / k as f64;
        let b = (1. / (n + 1) as f64).powi(2 * k);
        a * b
    }
    fn u(k: i32, n: i32) -> f64 {
        (0..n).map(|x| x + 1).map(|i| v(k, i)).sum()
    }
    fn S(k: i32, n: i32) -> f64 {
        (0..k).map(|x| x + 1).map(|i| u(i, n)).sum()
    }

    S(maxk, maxn)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fuzzy_equals(actual: f64, expected: f64) {
        let merr = 1.0e-10;
        let inrange = if expected == 0.0 {
            actual.abs() <= merr
        } else {
            (actual - expected).abs() / expected <= merr
        };
        if inrange == false {
            println!(
                "Expected value must be near: {:e} but was:{:e}",
                expected, actual
            );
        }
        assert_eq!(true, inrange);
    }

    fn dotest(maxk: i32, maxn: i32, exp: f64) -> () {
        assert_fuzzy_equals(doubles(maxk, maxn), exp);
    }

    #[test]
    fn basic_tests_doubles() {
        dotest(1, 3, 0.4236111111111111);
        dotest(1, 10, 0.5580321939764581);
        dotest(10, 100, 0.6832948559787737);

        dotest(1, 10, 0.5580321939764581);
        dotest(10, 1000, 0.6921486500921933);
        dotest(10, 10000, 0.6930471674194457);

    }
}
