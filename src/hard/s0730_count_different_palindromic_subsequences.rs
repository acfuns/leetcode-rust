use std::collections::VecDeque;

pub fn count_palindromic_subsequences(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mood = 1000000000 + 7;
    let mut f = vec![vec![1; n + 2]; n + 2];
    // 特判长度为 1 的区间
    for i in 1..=n {
        f[i][i] += 1;
    }
    // 枚举区间
    for len in 2..=n {
        let v = VecDeque::new();
        let mut q = [v.clone(), v.clone(), v.clone(), v.clone()];
        for i in 1..=n {
            q[(s[i - 1] - b'a') as usize].push_back(i);
            let j = i as i32 - len as i32 + 1;
            if j >= 1 {
                for k in 0..4 {
                    while !q[k].is_empty() && *q[k].front().unwrap() < (j as usize) {
                        q[k].pop_front().unwrap();
                    }
                    if !q[k].is_empty() {
                        f[j as usize][i] += 1;
                        let (l, r) = (q[k].front().unwrap(), q[k].back().unwrap());
                        if l < r {
                            f[j as usize][i] = (f[j as usize][i] + f[l + 1][r - 1]) % mood;
                        }
                    }
                }
            }
        }
    }
    (f[1][n] + mood - 1) % mood
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0730() {
        let s = String::from("abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba");
        assert_eq!(count_palindromic_subsequences(s), 104860361);
    }
}
