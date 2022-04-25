use rand::Rng;

struct Solution {
    arr: Vec<i32>,
}

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { arr: nums.clone() }
    }

    fn pick(&self, target: i32) -> i32 {
        let mut i = 0;
        let mut res = 0;

        for (idx, x) in self.arr.iter().enumerate() {
            if *x == target {
                i += 1;
                // 随机生成 [0, 1] 之间的数
                let mut rng = rand::thread_rng();
                // 注意这里fn gen_range<T, R>(&mut self, range: R)
                // 随机范围生成数是一个range参数，目前 rand 包版本是rand = "0.8.5"
                // 而 LeetCode 的 rand 版本是 0.7.2 所以要改一下参数
                // let j: i32 = rng.gen_range(1, i + 1);
                let j = rng.gen_range(1..=i);

                if j <= 1 {
                    res = idx as i32;
                }
            }
        }

        res
    }
}

// /**
//  * Your Solution object will be instantiated and called as such:
//  * let obj = Solution::new(nums);
//  * let ret_1: i32 = obj.pick(target);
//  */
// /**
//  * 测试麻烦，在此并不测试了
//  * 这题用的是经典的蓄水池抽样算法
//  */
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_0398() {
//         let nums = vec![1, 2, 3, 3, 3];
//         let obj = Solution::new(nums);
//         let ret_1: i32 = obj.pick(3);
//     }
// }
