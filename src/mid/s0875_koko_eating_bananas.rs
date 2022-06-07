pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let h_max = piles.iter().max().unwrap();
    let (mut l, mut r) = (0, *h_max + 1);
    while l + 1 != r {
        let mid = (l + r) >> 1;
        if check(&piles, mid) <= h {
            r = mid;
        } else {
            l = mid;
        }
    }
    r
}

fn check(piles: &Vec<i32>, v: i32) -> i32 {
    piles.iter().fold(0, |acc, x| acc + ((x - 1) / v + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0875() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        assert_eq!(min_eating_speed(piles, h), 30);
    }
}
