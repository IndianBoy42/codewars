fn dbl_linear(n: u32) -> u32 {
    let n = n as usize;

    let mut list = Vec::with_capacity(n * 2);
    list.push(1);

    let mut i = 0;
    let mut j = 1;
    while j <= n {
        let x = list[i];
        i += 1;

        let a = 2 * x + 1;
        let b = 3 * x + 1;
        if !list[i..].contains(&a) {
            list.push(a);
        }
        if !list[i..].contains(&b) {
            list.push(b);
        }

        list[j..].sort_unstable();

        j = list[j..]
            .iter()
            .copied()
            .position(|x| x > (2 * list[i] + 1))
            .map_or(list.len(), |x| x + j);
    }

    list[n]
}
#[cfg(test)]
mod tests {
    use super::dbl_linear;
    fn testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }

    #[test]
    fn basics_dbl_linear() {
        testing(10, 22);
        testing(20, 57);
        testing(30, 91);
        testing(50, 175);
        testing(1000000, 447);
    }
}
