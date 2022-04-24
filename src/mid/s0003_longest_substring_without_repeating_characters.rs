use std::cmp::max;

pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut res = 0;
    let mut tmp = [0; 300];
    while i < n {
        let k = s[i] as usize;
        tmp[k] += 1;
        if tmp[k] > 1 {
            while j <= i && tmp[k] > 1 {
                tmp[s[j] as usize] -= 1;
                j += 1;
            }
        }
        res = max(res, i - j + 1);
        i += 1;
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s: String = "abcabcbb".into();
        assert_eq!(3, length_of_longest_substring(s))
    }
}
