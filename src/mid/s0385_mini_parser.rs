#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub fn deserialize(s: String) -> NestedInteger {
    let mut u = 0;
    dfs(s.as_bytes(), &mut u)
}

fn dfs(s: &[u8], u: &mut usize) -> NestedInteger {
    let mut res = Vec::new();
    match s[*u] {
        b'[' => {
            *u += 1; // 跳过左括号
            while s[*u] != b']' {
                res.push(dfs(s, u));
            }
            *u += 1; // 跳过右括号
            if *u < s.len() && s[*u] == b',' {
                // 跳过逗号
                *u += 1;
            }
        }
        _ => {
            // println!("{}", u);
            let mut k = *u;
            while k < s.len() && s[k] != b',' && s[k] != b']' {
                k += 1;
            }
            let f = NestedInteger::Int(parse_to_int(&s[*u..k]));
            if k < s.len() && s[k] == b',' {
                // 跳过逗号
                k += 1;
            }
            *u = k;
            return f;
        }
    }
    NestedInteger::List(res)
}

fn parse_to_int(s: &[u8]) -> i32 {
    let mut res = 0;
    let mut sign = 1;
    for c in s {
        if *c == b'-' {
            sign = -1;
            continue;
        }
        if !c.is_ascii_digit() {
            break;
        }
        res = res * 10 + (*c - b'0') as i32;
    }
    res * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s0385_mini_parser() {
        assert_eq!(
            deserialize("[123,[456,[789]]]".to_string()),
            NestedInteger::List(vec![
                NestedInteger::Int(123),
                NestedInteger::List(vec![
                    NestedInteger::Int(456),
                    NestedInteger::List(vec![NestedInteger::Int(789)])
                ])
            ])
        );
    }
}
