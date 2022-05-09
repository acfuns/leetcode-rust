/// We iterate through the string, and if we see an 'I', we push the current value of i to the result
/// vector, and increment i. If we see a 'D', we push the current value of j to the result vector, and
/// decrement j. At the end, we push the current value of i to the result vector
///
/// Arguments:
///
/// * `s`: String
///
/// Returns:
///
/// a vector of integers.
pub fn di_string_match(s: String) -> Vec<i32> {
    let mut i: i32 = 0;
    let mut j = s.len() as i32;
    let mut res = Vec::new();
    s.chars()
        .map(|c| {
            if c.eq(&'I') {
                res.push(i);
                i += 1
            } else {
                res.push(j);
                j -= 1;
            }
        })
        .max();
    res.push(i);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0942() {
        let s = String::from("IDID");
        assert_eq!(di_string_match(s), vec![0, 4, 1, 3, 2]);
    }
}
