pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..nums.len() {
        let p = nums[i].abs() - 1;
        nums[p as usize] *= -1;
        if nums[p as usize] > 0 {
            res.push(nums[i].abs());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0442() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        assert_eq!(find_duplicates(nums), vec![2, 3]);
    }
}
