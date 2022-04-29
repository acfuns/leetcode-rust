const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

/// bfs
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if heights.is_empty() || heights[0].is_empty() {
        return vec![];
    }
    let n = heights.len();
    let m = heights[0].len();
    let mut st = vec![vec![0; n]; m];
    for i in 0..n {
        dfs(i, 0, 1, &mut st, &heights);
    }
    for i in 0..m {
        dfs(0, i, 1, &mut st, &heights);
    }
    for i in 0..n {
        dfs(i, m - 1, 2, &mut st, &heights);
    }
    for i in 0..m {
        dfs(n - 1, i, 2, &mut st, &heights);
    }

    let mut res: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        for j in 0..m {
            if st[i][j] == 3 {
                res.push(vec![i as i32, j as i32]);
            }
        }
    }
    res
}

fn dfs(x: usize, y: usize, t: i32, st: &mut Vec<Vec<i32>>, h: &Vec<Vec<i32>>) {
    if st[x][y] & t != 0 {
        return;
    }
    st[x][y] |= t;
    for i in 0..4 {
        let a = x as i32 + DX[i];
        let b = y as i32 + DY[i];
        if a >= 0
            && a < h.len() as i32
            && b >= 0
            && b < h[0].len() as i32
            && h[a as usize][b as usize] >= h[x][y]
        {
            dfs(a as usize, b as usize, t, st, h)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0417() {
        let heights = vec![vec![2, 1], vec![1, 2]];
        let ans = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
        assert_eq!(ans, pacific_atlantic(heights))
    }
}
