fn going1(n: i32) -> f64 {
    let v = (1..(n + 1))
        // .inspect(|x| print!("{} -> ", x))
        .map(|i| {
            ((i + 1)..(n + 1))
                .map(|j| (1.0f64 / (j as f64)))
                // .inspect(|x| print!("{} * ", x))
                .product::<f64>()
        })
        // .inspect(|x| println!(" = {}", x))
        .sum::<f64>();

    (v * 1e6).floor() / 1e6
}
fn going(n: i32) -> f64 {
    let v = (2..(n + 1))
        .rev()
        .fold((1.0f64, 1.0f64), |(total, mut elem), v| {
            elem /= v as f64;
            (total + elem, elem)
        })
        .0;

    (v * 1e6).floor() / 1e6
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fuzzy_equals(actual: f64, expected: f64) {
        let merr = 1.0e-6;
        let inrange = if expected == 0.0 {
            (actual.abs() <= merr)
        } else {
            ((actual - expected).abs() / expected <= merr)
        };
        if inrange == false {
            println!(
                "Expected value must be near: {:e} but was:{:e}",
                expected, actual
            );
        } else {
            //println!("....... GOOD\n");
        }
        assert_eq!(true, inrange);
    }

    fn dotest(n: i32, exp: f64) -> () {
        assert_fuzzy_equals(going(n), exp);
    }

    #[test]
    fn basics_going() {
        dotest(5, 1.275);
        dotest(6, 1.2125);
        dotest(7, 1.173214);
        dotest(8, 1.146651);
    }
}
