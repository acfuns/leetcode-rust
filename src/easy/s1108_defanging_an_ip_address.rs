// pub fn defang_i_paddr(address: String) -> String {
//     let mut res = String::from("");
//     for a in address.as_bytes() {
//         if let b'.' = a {
//             res.push_str("[.]");
//             continue;
//         }
//         res.push(*a as char);
//     }
//     res
// }

pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1108() {
        let address = "1.1.1.1".to_string();
        assert_eq!(defang_i_paddr(address), "1[.]1[.]1[.]1".to_string());
    }
}
