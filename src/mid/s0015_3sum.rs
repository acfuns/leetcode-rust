pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    nums.sort_unstable();
    let mut i = 0;
    let n = nums.len();
    while i < n {
        if i > 0 && nums[i] == nums[i - 1] {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            if j > i + 1 && nums[j] == nums[j - 1] {
                j += 1;
                continue;
            }
            let mut res = Vec::new();
            let flag = nums[i] + nums[j];
            while j < k && flag + nums[k] > 0 {
                k -= 1;
            }
            if j < k && flag + nums[k] == 0 {
                println!("{}, {}, {}", i, j, k);
                res.push(nums[i]);
                res.push(nums[j]);
                res.push(nums[k]);
                ans.push(res);
                k -= 1;
            }
            j += 1;
        }
        i += 1;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0015() {
        let nums = vec![1, 2, -2, -1];
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(res, three_sum(nums));
    }
}
