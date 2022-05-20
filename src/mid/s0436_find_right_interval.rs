pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let n = intervals.len();
    let mut idx: Vec<_> = intervals
        .iter()
        .enumerate()
        .map(|(i, x)| (x[0], x[1], i as i32))
        .collect();
    idx.sort_unstable();
    intervals
        .iter()
        .map(|x| {
            let (mut l, mut r): (i32, i32) = (-1, (n as i32));
            while l + 1 != r {
                let mid = (l + r) >> 1;
                if idx[mid as usize].0 >= x[1] {
                    r = mid;
                } else {
                    l = mid;
                }
            }
            if r == n as i32 {
                -1
            } else {
                idx[r as usize].2
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0436() {
        let intervals = vec![vec![3, 4], vec![2, 3], vec![1, 2]];
        assert_eq!(find_right_interval(intervals), vec![-1, 0, 1]);
    }
}
