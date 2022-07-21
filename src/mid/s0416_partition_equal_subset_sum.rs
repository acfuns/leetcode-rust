pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();

    if sum % 2 == 1 {
        return false;
    }
    let m = sum / 2;
    let mut f = vec![false; (m + 1) as usize];
    f[0] = true;
    for i in nums {
        for j in (i..=m).rev() {
            f[j as usize] |= f[(j - i) as usize];
        }
    }
    return f[m as usize];
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_0416() {
        let nums = vec![1, 2, 5];
        assert!(!can_partition(nums));
    }
}
