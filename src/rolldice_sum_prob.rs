use std::collections::HashMap;

fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    // ndice, sum -> ways
    let mut table = HashMap::<(i32, i32), i32>::new();

    for i in 1..=6 {
        table.insert((1, i), 1);
    }

    for ndice in 2..=dice_amount {
        for sumto in ndice..=(6 * ndice).min(sum) {
            let count = (1..=6)
                .map(|i| table.get(&(ndice - 1, sumto - i)).copied().unwrap_or(0))
                .sum();
            table.insert((ndice, sumto), count);
        }
    }

    (*table.get(&(dice_amount, sum)).unwrap() as f64) / (6f64.powf(dice_amount.into()))
}

fn assert_fuzzy_eq(actual: f64, expected: f64, eps: f64) {
    assert!(
        (actual - expected).abs() < eps,
        format!("Expected {}, but got {}", expected, actual)
    );
}

#[test]
fn returns_expected() {
    assert_fuzzy_eq(rolldice_sum_prob(11, 2), 0.055555555555, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob(8, 2), 0.13888888889, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob(8, 3), 0.0972222222222, 1e-10);
}
