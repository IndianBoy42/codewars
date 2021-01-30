fn order(sentence: &str) -> String {
    let mut words: Vec<_> = sentence.split_ascii_whitespace().collect();
    words.sort_by_key(|&word| word.chars().find(|&c| c.is_ascii_digit()).unwrap());
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(
            order("4of Fo1r pe6ople g3ood th5e the2"),
            "Fo1r the2 g3ood 4of th5e pe6ople"
        );
        assert_eq!(order(""), "");
    }
}
