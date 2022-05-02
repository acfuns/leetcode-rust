use std::collections::VecDeque;

const CDATA1: &str = "<![CDATA[";
const CDATA2: &str = "]]>";

pub fn is_valid(s: String) -> bool {
    let mut q = VecDeque::new();
    let n = s.len();
    let mut i: usize = 0;
    while i < n {
        if i + 8 < n && s[i..i + 9].eq(CDATA1) {
            if i == 0 {
                return false;
            }
            let mut j = i + 9;
            let mut ok = false;
            while j < n && !ok {
                if j + 2 < n && s[j..j + 3].eq(CDATA2) {
                    j += 3;
                    ok = true;
                } else {
                    j += 1;
                }
            }
            if !ok {
                return false;
            }
            i = j;
        } else if s[i..i + 1].starts_with('<') {
            if i == n - 1 {
                return false;
            }
            let is_end = s[i + 1..i + 2].starts_with('/');
            let p = if is_end { i + 2 } else { i + 1 };
            let mut j = p;
            while j < n && !s[j..j + 1].starts_with('>') {
                if s.as_bytes()[j] < 65 || s.as_bytes()[j] > 90 {
                    return false;
                }
                j += 1;
            }

            if j == n {
                return false;
            }
            let l = j - p;
            if !(1..=9).contains(&l) {
                return false;
            }
            let tag = &s[p..j];
            i = j + 1;
            if !is_end {
                q.push_back(tag);
            } else {
                if q.is_empty() || q.pop_back().unwrap() != tag {
                    return false;
                }
                if q.is_empty() && i < n {
                    return false;
                }
            }
        } else {
            if i == 0 {
                return false;
            }
            i += 1;
        }
    }
    q.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0591() {
        let code = String::from("<TRUe><![CDATA[123123]]]]><![CDATA[>123123]]></TRUe>");
        assert!(!is_valid(code));
    }
}
