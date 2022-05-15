use std::cmp::max;

// cross product
fn cross(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    x1 * y2 - y1 * x2
}

// get area
fn area(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> i32 {
    cross(b[0] - a[0], b[1] - a[1], c[0] - a[0], c[1] - a[1])
}

pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let mut res = 0;

    for a in &points {
        for b in &points {
            for c in &points {
                res = max(res, area(a, b, c));
            }
        }
    }
    res as f64 / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0812() {
        let points = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];
        assert_eq!(largest_triangle_area(points), 2.0);
    }
}
