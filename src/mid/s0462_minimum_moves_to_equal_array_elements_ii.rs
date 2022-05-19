/// Sort the array, then sum the absolute difference between each element and the median
///
/// Arguments:
///
/// * `nums`: the array of numbers
///
/// Returns:
///
/// The median of the array.
pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut res = 0;
    let n = nums.len();
    (0..n).for_each(|i| {
        res += (nums[i] - nums[n / 2]).abs();
    });
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0462() {
        let nums = vec![1, 10, 2, 9];
        assert_eq!(min_moves2(nums), 16);
    }
}
