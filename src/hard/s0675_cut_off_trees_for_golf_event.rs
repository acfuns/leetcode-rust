use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Tree {
    x: i32,
    y: i32,
    h: i32,
}

impl Tree {
    fn new(x: i32, y: i32, h: i32) -> Tree {
        Tree { x, y, h }
    }
}

fn bfs(start: Tree, end: Tree, n: i32, m: i32, g: Vec<Vec<i32>>) -> i32 {
    // 特判
    if start.x == end.x && start.y == end.y {
        return 0;
    }
    let mut q = VecDeque::new();
    let inf: i32 = 0x3f3f3f3f;
    let mut dist = vec![vec![inf; m as usize]; n as usize];
    dist[start.x as usize][start.y as usize] = 0;
    q.push_back(start);
    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, 1, 0, -1];
    while !q.is_empty() {
        let t = q.pop_front().unwrap();
        for i in 0..4 {
            let x = t.x + dx[i];
            let y = t.y + dy[i];
            if x >= 0 && x < n && y >= 0 && y < m && g[x as usize][y as usize] > 0 {
                if dist[x as usize][y as usize] > dist[t.x as usize][t.y as usize] + 1 {
                    dist[x as usize][y as usize] = dist[t.x as usize][t.y as usize] + 1;
                    if x == end.x && y == end.y {
                        return dist[x as usize][y as usize];
                    }
                    q.push_back(Tree::new(x, y, g[x as usize][y as usize]));
                }
            }
        }
    }
    -1
}

pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
    let (n, m) = (forest.len(), forest[0].len());
    let mut trs = Vec::new();
    (0..n).for_each(|i| {
        (0..m).for_each(|j| {
            if forest[i][j] > 1 {
                trs.push(Tree::new(i as i32, j as i32, forest[i][j]));
            }
        });
    });
    trs.sort_unstable_by(|a, b| a.h.cmp(&b.h));

    // 抽象化排序后每个点到下一个点的最短距离之和
    let mut res = 0;
    let mut last = Tree::new(0, 0, forest[0][0]);
    for tr in trs {
        let t = bfs(last, tr, n as i32, m as i32, forest.clone());
        if t == -1 {
            return -1;
        }
        res += t;
        last = tr;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0675() {
        let forest = vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]];
        assert_eq!(cut_off_tree(forest), 6);
    }
}
