pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut ans = Vec::new();
    let n = words.len();
    let mut i: usize = 0;

    while i < n {
        let mut list = Vec::new();
        let mut cur = 0;
        list.push(&words[i]);
        cur += words[i].len();
        i += 1;
        while i < n && cur + words[i].len() <= max_width as usize - 1 {
            list.push(&words[i]);
            cur += words[i].len() + 1;
            i += 1;
        }

        let l = list.len();
        let more_space_num = max_width as usize - cur;
        let mut s = String::from("");
        s.push_str(list[0]);

        // 最后一行置左特殊处理
        if i == n {
            for x in list.into_iter().skip(1) {
                s.push(' ');
                s.push_str(x);
            }
            s.push_str(" ".repeat(more_space_num).as_str());
            ans.push(s);
            break;
        }

        // 只有一个单词置左处理
        if l == 1 {
            s.push_str(" ".repeat(more_space_num).as_str());
            ans.push(s);
            continue;
        }

        let mut space_width = max_width as usize - (cur - l + 1);
        let mut gap_count = l - 1;
        // 多个单词普遍情况
        for (idx, x) in list.into_iter().enumerate() {
            if idx > 0 {
                s.push_str(x);
            }
            if idx == l - 1 {
                break;
            }
            let gap_width = space_width / gap_count;
            let cnt = gap_width * gap_count;
            gap_count -= 1;
            if cnt < space_width {
                s.push_str(" ".repeat(gap_width + 1).as_str());
                space_width -= gap_width + 1;
            } else {
                s.push_str(" ".repeat(gap_width).as_str());
                space_width -= gap_width;
            }
        }
        ans.push(s);
    }

    ans
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_0068() {
        let words: Vec<String> = vec![
            "Science".into(),
            "is".into(),
            "what".into(),
            "we".into(),
            "understand".into(),
            "well".into(),
            "enough".into(),
            "to".into(),
            "explain".into(),
            "to".into(),
            "a".into(),
            "computer.".into(),
            "Art".into(),
            "is".into(),
            "everything".into(),
            "else".into(),
            "we".into(),
            "do".into(),
        ];

        let max_width = 20;
        let ans = full_justify(words, max_width);
        assert_eq!(
            ans,
            vec![
                "Science  is  what we".to_string(),
                "understand      well".to_string(),
                "enough to explain to".to_string(),
                "a  computer.  Art is".to_string(),
                "everything  else  we".to_string(),
                "do                  ".to_string()
            ]
        )
    }
}
