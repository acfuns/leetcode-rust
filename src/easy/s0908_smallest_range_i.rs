pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    (nums.iter().max().unwrap() - nums.iter().min().unwrap() - 2 * k).max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0908() {
        let nums = vec![0, 10];
        let k = 2;
        assert_eq!(6, smallest_range_i(nums, k));
    }
}
