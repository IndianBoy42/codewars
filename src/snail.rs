fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let n = matrix.len();

    if n == 0 || matrix[0].len() == 0 {
        return Vec::new();
    }

    (0usize..)
        .flat_map(|i| match (i % 4, i / 4) {
            (0, j) => {
                // top moving right
                (j..(n - j)).map(|c| matrix[j][c]).collect::<Vec<_>>()
            }
            (1, j) => {
                // right moving down
                ((j + 1)..(n - j))
                    .map(|c| matrix[c][n - j - 1])
                    .collect::<Vec<_>>()
            }
            (2, j) => {
                // bottom moving left
                (j..(n - j - 1))
                    .rev()
                    .map(|c| matrix[n - j - 1][c])
                    .collect::<Vec<_>>()
            }
            (3, j) => {
                // left moving up
                ((j + 1)..(n - j - 1))
                    .rev()
                    .map(|c| matrix[c][j])
                    .collect::<Vec<_>>()
            }
            _ => unreachable!(),
        })
        .take(n * n)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
