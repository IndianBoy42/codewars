fn swap_lt(a: i32, b: i32) -> (i32, i32) {
    (i32::min(a, b), i32::max(a, b))
}

fn sq_in_rect(length: i32, width: i32) -> Option<Vec<i32>> {
    if length == width {
        return None;
    }
    let res = (0..).try_fold(
        (Vec::new(), swap_lt(length, width)),
        |(mut list, (l, w)), _| {
            dbg!(&list, l, w);
            if l != 0 {
                list.push(l);
                Ok((list, swap_lt(l, w - l)))
            } else {
                Err((list, (l, w)))
            }
        },
    );
    if let Err((list, (l, w))) = res {
        dbg!(&list, l, w);
        Some(list)
    } else {
        None
    }
}

fn sq_in_rect2(length: i32, width: i32) -> Option<Vec<i32>> {
    if length == width {
        return None;
    }
    let mut rect = swap_lt(length, width);
    let list = std::iter::from_fn(|| {
        let (l, w) = rect;
        if l == 0 {
            None
        } else {
            rect = swap_lt(l, w-l);
            Some(l)
        }
    }).collect();
    Some(list)
}

fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq_in_rect() {
    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);
}
