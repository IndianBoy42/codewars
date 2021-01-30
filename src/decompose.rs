fn decompose(n: i64) -> Option<Vec<i64>> {
    let n = n as usize;
    let mut dp = vec![0; (n * n) + 1];
    let sqs = (1..).map(|x| x * x);
    for i in 0..n {
        dp[i * i] = i;
    }

    for i in 1..(n * n) {
        // let isqrt = (i as f64).sqrt() as usize;
        for (_j, sq) in sqs.clone().take_while(|&x| x < i).enumerate() {
            if dp[i - sq] != 0 {}
        }
        for (j, _p) in dp[..i].iter().copied().enumerate().rev() {
            let _make = i - j;
        }
    }

    unimplemented!()
}

fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
    assert_eq!(decompose(n), exp)
}

#[test]
fn tests_decompose() {
    testing(50, Some(vec![1, 3, 5, 8, 49]));
    testing(44, Some(vec![2, 3, 5, 7, 43]));
    testing(625, Some(vec![2, 5, 8, 34, 624]));
    testing(5, Some(vec![3, 4]));
}
