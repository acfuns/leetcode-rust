use std::collections::HashSet;

pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let mut q = HashSet::new();
    for x in nums {
        if q.contains(&x) {
            return x;
        }
        q.insert(x);
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0961() {
        let nums = vec![1, 2, 3, 3];
        assert_eq!(repeated_n_times(nums), 3);
    }
}
