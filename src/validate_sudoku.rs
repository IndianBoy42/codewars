use std::collections::HashMap;

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn is_valid(&self) -> bool {
        let n = self.data.len();
        if n > 0 && self.data[0].len() == 0 {
            return false;
        }
        let nsqrt = (self.data.len() as f32).sqrt() as usize;
        let mut rowfill: HashMap<usize, Vec<u32>> = HashMap::with_capacity(self.data.len());
        let mut boxfill: HashMap<usize, Vec<u32>> = HashMap::with_capacity(self.data.len());
        let mut colfill: HashMap<usize, Vec<u32>> = HashMap::with_capacity(self.data.len());
        let elements = self
            .data
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().copied().enumerate().map(move |(j, e)| (i, j, e)));

        for (i, j, e) in elements {
            rowfill
                .entry(i)
                .and_modify(|v| v.push(e))
                .or_insert(vec![e]);
            colfill
                .entry(j)
                .and_modify(|v| v.push(e))
                .or_insert(vec![e]);
            boxfill
                .entry((j / nsqrt) + (i / nsqrt) * nsqrt)
                .and_modify(|v| v.push(e))
                .or_insert(vec![e]);
        }

        rowfill.values_mut().all(|vec| {
            vec.sort();
            vec.iter()
                .copied()
                .zip(1..=n)
                .all(|(a, b)| (a as usize) == b)
        }) && colfill.values_mut().all(|vec| {
            vec.sort();
            vec.iter()
                .copied()
                .zip(1..=n)
                .all(|(a, b)| (a as usize) == b)
        }) && boxfill.values_mut().all(|vec| {
            vec.sort();
            vec.iter()
                .copied()
                .zip(1..=n)
                .all(|(a, b)| (a as usize) == b)
        })
    }
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
        ],
    };

    let good_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],
            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ],
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ],
    };

    let bad_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1],
        ],
    };
    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
}
