pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut cnt = 0;
    let m = strs[0].len();
    for i in 0..m {
        let mut f = strs[0].chars().nth(i).unwrap();
        for s in strs.iter().skip(1) {
            let c = s.chars().nth(i).unwrap();
            if f > c {
                cnt += 1;
                break;
            }
            f = c;
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0944() {
        let strs = vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()];
        assert_eq!(min_deletion_size(strs), 1);
    }
}
