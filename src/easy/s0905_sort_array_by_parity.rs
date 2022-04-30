pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < n {
        if nums[i] & 1 == 0 {
            nums.swap(i, j);
            j += 1;
        }
        i += 1;
    }
    nums
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0905() {
        let nums = vec![3, 1, 2, 4];
        assert_eq!(vec![2, 4, 3, 1], sort_array_by_parity(nums))
    }
}
