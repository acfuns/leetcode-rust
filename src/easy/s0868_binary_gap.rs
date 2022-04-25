use std::cmp::max;

pub fn binary_gap(n: i32) -> i32 {
    let mut n = n;
    let mut arr: Vec<i32> = Vec::new();
    let mut i = 0;

    while n != 0 {
        if n % 2 == 1 {
            arr.push(i);
        }
        n /= 2;
        i += 1;
    }
    let l = arr.len();
    if l <= 1 {
        return 0;
    }
    let mut res = 0;

    for j in 1..l {
        res = max(res, arr[j] - arr[j - 1]);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0868() {
        let ans = binary_gap(22);
        assert_eq!(ans, 2);
    }
}
