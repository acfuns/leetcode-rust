use std::cmp::max;

/// It returns the cross product of two vectors
///
/// Arguments:
///
/// * `x1`: x coordinate of the first point
/// * `y1`: The y-coordinate of the first point.
/// * `x2`: i32, y2: i32
/// * `y2`: i32, x2: i32, y1: i32, x1: i32
///
/// Returns:
///
/// The cross product of the two vectors.
fn cross(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    x1 * y2 - y1 * x2
}

// get area
/// It takes three points and returns the area of the triangle formed by them
///
/// Arguments:
///
/// * `a`: &Vec<i32> - a reference to a vector of i32s
/// * `b`: &Vec<i32> - a reference to a vector of i32s
/// * `c`: &Vec<i32>
///
/// Returns:
///
/// The area of the triangle formed by the three points.
fn area(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> i32 {
    cross(b[0] - a[0], b[1] - a[1], c[0] - a[0], c[1] - a[1])
}

/// For each triplet of points, compute the area of the triangle formed by them and return the maximum
///
/// Arguments:
///
/// * `points`: a list of points, each point is a list of two integers
///
/// Returns:
///
/// The area of the largest triangle that can be formed from the given points.
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
