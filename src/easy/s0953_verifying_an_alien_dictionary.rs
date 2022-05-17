use std::{cmp::Ordering, collections::HashMap};

pub fn is_alien_sorted(mut words: Vec<String>, order: String) -> bool {
    let mut hp = HashMap::new();
    let _ = order
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, c)| hp.insert(c, i))
        .collect::<Vec<_>>();

    let mut words_clone = words.clone();
    words_clone.sort_unstable_by(|a, b| {
        let (s1, s2) = (a.as_bytes(), b.as_bytes());

        let (n, m) = (s1.len(), s2.len());

        let (mut i, mut j) = (0, 0);
        while i < n && j < m {
            if s1[i] != s2[i] {
                return hp[&s1[i]].cmp(&hp[&s2[i]]);
            }
            i += 1;
            j += 1;
        }
        if i < n {
            return Ordering::Greater;
        }
        if j < m {
            return Ordering::Less;
        }
        Ordering::Less
    });

    words.eq(&words_clone)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0953() {
        let words = vec!["hello".to_string(), "leetcode".to_string()];
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        assert!(is_alien_sorted(words, order));
    }
}
