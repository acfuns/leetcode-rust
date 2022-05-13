use std::{cmp::max, collections::VecDeque};

pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let mut s1 = s1.bytes().collect::<VecDeque<_>>();
    let mut s2 = s2.bytes().collect::<VecDeque<_>>();
    let sum1: i32 = s1.iter().map(|&x| x as i32).sum();
    let sum2: i32 = s2.iter().map(|&x| x as i32).sum();

    let n = s1.len();
    let m = s2.len();
    let mut f: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];

    s1.push_front(0);
    s2.push_front(0);

    (1..=n).for_each(|i| {
        (1..=m).for_each(|j| {
            f[i][j] = max(f[i - 1][j], f[i][j - 1]);
            if s1[i] == s2[j] {
                f[i][j] = max(f[i][j], f[i - 1][j - 1] + s1[i] as i32 + s2[j] as i32);
            }
        });
    });

    sum1 + sum2 - f[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0712() {
        let s1 = "sea".to_string();
        let s2 = "eat".to_string();
        assert_eq!(minimum_delete_sum(s1, s2), 231);
    }
}
