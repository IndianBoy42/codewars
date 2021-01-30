use std::collections::BTreeSet;

fn prod_merge(a: BTreeSet<i64>, b: &BTreeSet<i64>) -> BTreeSet<i64> {
    let a = a.iter().copied();
    let b = b.iter().copied();
    a.flat_map(|i| b.clone().map(move |j| (i, j)))
        .map(|(a, b)| a * b)
        .collect()
}

fn prod_one(v: &Vec<BTreeSet<i64>>, n: i64) -> BTreeSet<i64> {
    if n == 1 {
        return [1].iter().copied().collect();
    } else {
        let partitions = (0..(n / 2)).map(|x| x + 1).map(|i| (i, n - i));
        let mut set = partitions
            .map(|(i, j)| (&v[i as usize - 1], &v[j as usize - 1]))
            .map(|(a, b)| prod_merge(a.clone(), b))
            .fold(BTreeSet::new(), |mut set, ins| {
                set.extend(ins);
                set
            });
        set.insert(n);
        set
    }
}

fn prod(n: i64) -> Vec<i64> {
    let sets = (0..n)
        .map(|x| x + 1)
        .fold(Vec::with_capacity(n as usize), |mut vec, x| {
            vec.push(prod_one(&vec, x));
            vec
        });

    sets.last().unwrap().iter().copied().collect()
}

fn part(n: i64) -> String {
    let prodvals = prod(n);

    let mean = prodvals.iter().copied().sum::<i64>() as f64 / prodvals.len() as f64;
    let range = prodvals.last().unwrap() - prodvals.first().unwrap();
    let median = if prodvals.len() % 2 == 1 {
        prodvals[prodvals.len() / 2] as f64
    } else {
        (prodvals[prodvals.len() / 2] + prodvals[prodvals.len() / 2 - 1]) as f64 / 2f64
    };

    format!(
        "Range: {} Average: {:.02} Median: {:.02}",
        range, mean, median
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testequal(ans: &str, sol: &str) {
        assert_eq!(ans, sol, "Expected \"{}\", got \"{}\".", sol, ans);
    }

    #[test]
    fn test_prod() {
        assert_eq!(prod(5), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(prod(8), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 16, 18]);
    }

    #[test]
    fn returns_expected() {
        testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
        testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
        testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
        testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
        testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
        dbg!(part(50));
    }
}
