pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut i = 0;
    let mut j = 0;
    let mut p = 1;
    let mut res = 0;
    while i < n {
        p *= nums[i];
        while j <= i && p >= k {
            p /= nums[j];
            j += 1;
        }
        res += i - j + 1;
        i += 1;
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0713() {
        let nums = vec![10, 5, 2, 6];
        let k = 100;
        assert_eq!(num_subarray_product_less_than_k(nums, k), 8);
    }
}
