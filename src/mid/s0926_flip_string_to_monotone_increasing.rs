use std::cmp::min;

pub fn min_flips_mono_incr(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut pre_sum = vec![0; n + 1];
    for i in 1..=n {
        pre_sum[i] = pre_sum[i - 1] + (s[i - 1] - b'0') as i32;
    }

    let mut res = n as i32 - pre_sum[n];

    for i in 1..=n {
        res = min(
            res,
            pre_sum[i] + n as i32 - i as i32 - pre_sum[n] + pre_sum[i],
        );
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0926() {
        let s = String::from("00110");
        assert_eq!(min_flips_mono_incr(s), 1);
        assert_eq!(min_flips_mono_incr("010110".into()), 2);
        assert_eq!(min_flips_mono_incr("00011000".into()), 2);
    }
}
