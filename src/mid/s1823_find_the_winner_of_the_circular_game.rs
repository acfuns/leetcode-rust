// k = 2, n = 5

// f(n-1, m)
// 0 1 2 3 4
// 0 1 2 3
// 0 1 2
// 0 1
// 0

// f'(n-1, m)
// 0 1 2 3 4
// 2 3 4 0
// 4 0 2
// 2 4
// 2

// f(n, m) = (f(n - 1, m) - m) % n; (n > 1)
//         = 0； （n == 1)

// 都是根据原始的f来映射的，推出递推公式，知道最后有序状态，那么映射关系就一一对应了
// 每一轮都找到胜出者在上一轮中的编号变化之后的编号

use std::collections::VecDeque;

pub fn find_the_winner(n: i32, k: i32) -> i32 {
    // # 递归 时间复杂度 O(n) 空间复杂度 O(n)
    // if n <= 1 {
    //     return n;
    // }
    // let ans = (find_the_winner(n - 1, k) + k) % n;
    // if ans == 0 {
    //     n
    // } else {
    //     ans
    // }
    // 迭代 时间复杂度 O(n) 空间复杂度 O(1)
    let mut ans = 0;
    for i in 2..=n {
        ans = (ans + k) % i;
    }
    ans + 1
    // 队列模拟 时间复杂度 O(n^2) 空间复杂度 O(n)
    // let mut q = VecDeque::new();
    // for i in 1..=n {
    //     q.push_back(i);
    // }
    // while q.len() > 1 {
    //     for _ in 0..k - 1 {
    //         q.push_back(*q.front().unwrap());
    //         q.pop_front();
    //     }
    //     q.pop_front();
    // }
    // *q.front().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1823() {
        assert_eq!(find_the_winner(5, 2), 3);
    }
}
