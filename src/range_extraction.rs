mod solution {
    pub fn range_extraction(a: &[i32]) -> String {
        use std::fmt::Write;
        let mut it = a.iter().copied().peekable();
        let mut out = String::with_capacity(a.len() * 5);
        let mut list = Vec::with_capacity(a.len());
        while let Some(num) = it.next() {
            if let Some(&next) = it.peek() {
                if (next - num) > 1 {
                    write!(&mut out, "{},", num).unwrap();
                } else {
                    list.clear();
                    list.push(num);
                    while let Some(&more) = it.peek() {
                        let prev = list.last().unwrap();
                        if (more - prev) > 1 {
                            break;
                        } else {
                            list.push(it.next().unwrap());
                        }
                    }
                    dbg!(&list);
                    if list.len() == 2 {
                        write!(&mut out, "{},{},", num, next).unwrap();
                    } else {
                        write!(&mut out, "{}-{},", num, list.last().unwrap()).unwrap();
                    }
                }
            } else {
                write!(&mut out, "{},", num).unwrap();
            }
        }
        out.trim_end_matches(',').into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }
}
