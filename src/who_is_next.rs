/// _BigBang_ gang
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Name {
    Sheldon,
    Leonard,
    Penny,
    Rajesh,
    Howard,
}

/// Just to make code look a bit nicer
type Names = Vec<Name>;

fn generation(n: usize) -> (usize, usize, usize) {
    (0..)
        .map(|i| (i as usize, 2usize.pow(i)))
        .map(|(i, th)| (i + 1, 5 * th))
        .scan(0, |acc, (i, size)| {
            *acc += size;
            Some((i, *acc, *acc - size, size / 5))
        })
        .find(|&(_, thresh, _, _)| thresh >= n)
        .map(|(i, _, t, s)| (i, t, s))
        .unwrap()
}

/// Will return the `Name` of the person who will drink the `n`-th cola.
fn who_is_next(_names: &Names, n: usize) -> Name {
    let (g, th, sz) = generation(n);
    let i = (n - th + sz - 1) / sz;
    println!("n{} gen{} thresh{} i{} sz{}", n, g, th, i, sz);
    [
        Name::Sheldon, // Dummy
        Name::Sheldon,
        Name::Leonard,
        Name::Penny,
        Name::Rajesh,
        Name::Howard,
    ][i]
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];

        assert_eq!(who_is_next(names, 1), Name::Sheldon);
        assert_eq!(who_is_next(names, 6), Name::Sheldon);
        assert_eq!(who_is_next(names, 52), Name::Penny);
        assert_eq!(who_is_next(names, 7_230_702_951), Name::Leonard);
    }

    #[test]
    fn test_alot() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];

        for (i, name) in (1..)
            .flat_map(|i| {
                (0..5).flat_map(move |j| std::iter::repeat(names[j]).take(2usize.pow(i - 1)))
            })
            .enumerate()
            .nth(759137-1)
        {
            println!("{:?}", name);
            assert_eq!(who_is_next(names, i + 1), name, "i: {}", i + 1);
        }
    }

    #[test]
    fn test_gen() {
        for i in 1..50 {
            let expected = if i <= 5 {
                (1, 0, 1)
            } else if i <= 15 {
                (2, 5, 2)
            } else if i <= 35 {
                (3, 15, 4)
            } else if i <= 75 {
                (4, 35, 8)
            } else {
                unreachable!()
            };
            assert_eq!(generation(i), expected, "i: {}", i);
        }
    }
}
