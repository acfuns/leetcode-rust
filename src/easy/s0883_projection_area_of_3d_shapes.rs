use std::cmp::max;

pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let mut xy = 0;
    let mut xz = [0; 60];
    let mut yz = [0; 60];
    let mut res = 0;
    let n = grid.len();
    for (i, x) in grid.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            if *y > 0 {
                xy += 1;
            }
            xz[i] = max(xz[i], *y);
            yz[j] = max(yz[j], *y);
            if i == n - 1 {
                res += yz[j];
            }
        }
        res += xz[i];
    }
    res + xy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0883() {
        let ans = projection_area(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(ans, 17);
    }
}
