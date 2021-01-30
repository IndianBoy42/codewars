fn rgb(r: i32, g: i32, b: i32) -> String {
    fn clamp(x: i32) -> u8 {
        x.min(255).max(0) as _
    }
    format!("{:02X}{:02X}{:02X}", clamp(r), clamp(g), clamp(b))
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    macro_rules! compare {
        ( $got : expr, $expected : expr ) => {
            if $got != $expected {
                panic!("Got: {}\nExpected: {}\n", $got, $expected);
            }
        };
    }
    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
