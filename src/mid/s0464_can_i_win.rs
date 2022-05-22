pub fn can_i_win(n: i32, m: i32) -> bool {
    // 判断边界，max小于总数一定不存在正确的结果
    if n * (n + 1) / 2 < m {
        return false;
    }
    let mut f = vec![-1; 1 << n];

    dp(0, n, m, &mut f)
}

pub fn dp(x: i32, n: i32, m: i32, f: &mut Vec<i32>) -> bool {
    if f[x as usize] != -1 {
        return f[x as usize] != 0;
    }
    let mut sum = 0;
    (0..n).for_each(|i| {
        if x >> i & 1 == 1 {
            sum += i + 1;
        }
    });
    for i in 0..n {
        if x >> i & 1 == 0 {
            // 当前满足了必胜条件
            if sum + i + 1 >= m {
                f[x as usize] = 1;
                return true;
            }
            // 回溯回来必败的话, 那么当前状态必胜
            if !dp(x + (1 << i), n, m, f) {
                f[x as usize] = 1;
                return true;
            }
        }
    }
    // 如果都是必胜的，那么当前状态就必败
    f[x as usize] = 0;
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0464() {
        let max_choosable_integer = 10;
        let desired_total = 11;
        assert!(!can_i_win(max_choosable_integer, desired_total))
    }
}
