use std::collections::HashMap;

pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    let mut res = Vec::new();

    for w in words {
        if check(&w, &pattern) {
            res.push(w);
        }
    }
    return res;
}

pub fn check(w: &String, p: &String) -> bool {
    let (mut f, mut g) = (HashMap::new(), HashMap::new());
    let w = w.as_bytes();
    let p = p.as_bytes();
    let n = p.len();
    for i in 0..n {
        if f.contains_key(&w[i]) && *f.get(&w[i]).unwrap() != p[i] {
            return false;
        }
        if g.contains_key(&p[i]) && *g.get(&p[i]).unwrap() != w[i] {
            return false;
        }
        f.insert(w[i], p[i]);
        g.insert(p[i], w[i]);
    }
    return true;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_0890() {
        let words = vec![
            "abc".to_string(),
            "deq".to_string(),
            "mee".to_string(),
            "aqq".to_string(),
            "dkd".to_string(),
            "ccc".to_string(),
        ];
        let pattern = "abb".to_string();
        assert_eq!(
            find_and_replace_pattern(words, pattern),
            vec!["mee".to_string(), "aqq".to_string()]
        );
    }
}
