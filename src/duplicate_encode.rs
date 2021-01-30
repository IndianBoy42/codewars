fn duplicate_encode(word: &str) -> String {
    let counts =
        word.bytes()
            .map(|b| b.to_ascii_lowercase())
            .fold(vec![0; 128], |mut counts, b| {
                counts[b as usize] += 1;
                counts
            });

    word.bytes()
        .map(|b| {
            if counts[b.to_ascii_lowercase() as usize] > 1 {
                ')'
            } else {
                '('
            }
        })
        .collect()
}
#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}
