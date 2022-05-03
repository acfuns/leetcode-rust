use std::cmp::Ordering;

pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
    logs.sort_by(|a, b| {
        let (s1, s2) = (
            a.splitn(2, ' ').collect::<Vec<_>>(),
            b.splitn(2, ' ').collect::<Vec<_>>(),
        );
        match (
            s1[1].as_bytes()[0].is_ascii_digit(),
            s2[1].as_bytes()[0].is_ascii_digit(),
        ) {
            (true, true) => Ordering::Equal,
            (false, false) => (s1[1], s1[0]).cmp(&(s2[1], s2[0])),
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
        }
    });
    logs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0937() {
        let logs: Vec<String> = vec![
            "dig1 8 1 5 1".into(),
            "let1 art can".into(),
            "dig2 3 6".into(),
            "let2 own kit dig".into(),
            "let3 art zero".into(),
        ];
        assert_eq!(
            reorder_log_files(logs),
            vec![
                "let1 art can".to_string(),
                "let3 art zero".to_string(),
                "let2 own kit dig".to_string(),
                "dig1 8 1 5 1".to_string(),
                "dig2 3 6".to_string()
            ]
        );
    }
}
