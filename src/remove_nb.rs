fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    // your code
    let total: i32 = (1..=m).sum();
    let combs = (1..=m).flat_map(|i| (i..=m).map(move |j| (i, j)));
    let mut ans: Vec<_> = combs
        .filter(|&(i, j)| {
            let prod = i * j;
            let sum = total - i - j;
            // dbg!(i, j, prod, sum);
            prod == sum
        })
        .collect();
    ans.extend(ans.clone().iter().map(|&(i,j)| (j,i)));
    ans
}

fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
    assert_eq!(remove_nb(n), exp)
}

#[test]
fn basics_remove_nb() {
    testing(26, vec![(15, 21), (21, 15)]);
    testing(100, vec![]);
    testing(101, vec![(55, 91), (91, 55)]);
    testing(102, vec![(70, 73), (73, 70)]);
}
