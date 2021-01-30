fn rot(ch: u8, base: u8, len: u8, n: u8) -> u8 {
    ((ch - base + n) % len) + base
}
fn play_pass(s: &str, n: u32) -> String {
    // your code
    let v: Vec<_> = s
        .bytes()
        .enumerate()
        .map(|(i, b)| {
            if i % 2 == 0 {
                b.to_ascii_uppercase()
            } else {
                b.to_ascii_lowercase()
            }
        })
        .map(|b| match b {
            b'A'..=b'Z' => rot(b, b'A', 26, n as _),
            b'a'..=b'z' => rot(b, b'a', 26, n as _),
            b'0'..=b'9' => (9 - (b - b'0')) + b'0',
            _ => b,
        })
        .rev()
        .collect();
    String::from_utf8(v).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, n: u32, exp: &str) -> () {
        println!(" s: {:?};", s);
        println!("n: {:?};", n);
        let ans = play_pass(s, n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest("I LOVE YOU!!!", 1, "!!!vPz fWpM J");
        dotest("I LOVE YOU!!!", 0, "!!!uOy eVoL I");
        dotest("BORN IN 2015!", 1, "!4897 Oj oSpC");
        dotest("AAABBCCY", 1, "zDdCcBbB");
        dotest(
            "MY GRANMA CAME FROM NY ON THE 23RD OF APRIL 2015",
            2,
            "4897 NkTrC Hq fT67 GjV Pq aP OqTh gOcE CoPcTi aO",
        );
    }
}
