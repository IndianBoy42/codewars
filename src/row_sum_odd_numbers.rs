fn nth_odd(n: i64) -> i64 {
    n * (n - 1) + 1
}

fn row_sum_odd_numbers2(n: i64) -> i64 {
    (nth_odd(n)..).step_by(2).take(n as usize).sum()
}

fn sum_n_even_numbers(n: i64) -> i64 {
    n * (n + 1)
}

fn row_sum_odd_numbers(n: i64) -> i64 {
    dbg!(n);
    dbg!(sum_n_even_numbers(n - 1));
    dbg!(nth_odd(n));
    sum_n_even_numbers(n - 1) + n * nth_odd(n)
}

#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(2), 3 + 5);
    assert_eq!(row_sum_odd_numbers(3), 7 + 9 + 11);
    assert_eq!(row_sum_odd_numbers(4), 13 + 15 + 17 + 19);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}
