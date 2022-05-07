use std::collections::{HashMap, HashSet, VecDeque};

pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
    let mut S = HashSet::new();
    for s in bank.iter() {
        S.insert(s);
    }
    let mut dist = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back(start.clone());
    dist.insert(start, 0);
    let chrs = ['A', 'T', 'C', 'G'];

    while !q.is_empty() {
        let t = q.pop_front().unwrap();
        let mut i = 0;
        while i < t.len() {
            let s = t.clone();
            for c in chrs {
                let f: String = s
                    .chars()
                    .enumerate()
                    .map(|(j, x)| if j == i { c } else { x })
                    .collect();
                // println!("{}", f);
                if S.contains(&f) && !dist.contains_key(&f) {
                    dist.insert(f.clone(), dist[&t] + 1);
                    if f.eq(&end) {
                        println!("{}", f);
                        return dist[&t] + 1;
                    }
                    q.push_back(f.clone())
                }
            }
            i += 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0433() {
        let start: String = "AACCGGTT".into();
        let end: String = "AAACGGTA".into();
        let bank = vec![
            "AACCGGTA".to_string(),
            "AACCGCTA".to_string(),
            "AAACGGTA".to_string(),
        ];
        assert_eq!(min_mutation(start, end, bank), 2);
    }
}
