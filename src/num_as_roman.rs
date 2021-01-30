/// Converts a number to a string representating roman numeral.
fn digits(num: i32) -> [i32; 10] {
    let mut digits = [0; 10];
    digits.iter_mut().fold(num, |left, digit| {
        *digit = left % 10;
        left / 10
    });
    digits
}

fn num_as_roman(num: i32) -> String {
    use std::collections::HashMap;
    let _map = {
        let mut map = HashMap::new();
        map.insert("I", 1);
        map.insert("V", 5);
        map.insert("X", 10);
        map.insert("L", 50);
        map.insert("C", 100);
        map.insert("D", 500);
        map.insert("M", 1_000);
        map
    };
    let triplets = [
        ["I", "V", "X"],
        ["X", "L", "C"],
        ["C", "D", "M"],
        ["M", "M", "M"],
    ];
    let digit_map: [&[usize]; 10] = [
        &[],           // 0
        &[0],          // 1
        &[0, 0],       // 2
        &[0, 0, 0],    // 3
        &[0, 1],       // 4
        &[1],          // 5
        &[1, 0],       // 6
        &[1, 0, 0],    // 7
        &[1, 0, 0, 0], // 8
        &[0, 2],       // 9
    ];

    if num > 4_000 {
        panic!("IDK");
    }

    let digits = digits(num);

    let v: Vec<&str> = digits
        .iter()
        .copied()
        .zip(&triplets)
        .rev()
        .flat_map(|(digit, triplet)| {
            digit_map[digit as usize]
                .iter()
                .copied()
                .map(move |i| triplet[i])
        })
        .collect();
    v.join("")
}

#[test]
fn returns_expected() {
    assert_eq!(num_as_roman(182), "CLXXXII");
    assert_eq!(num_as_roman(1990), "MCMXC");
    assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
