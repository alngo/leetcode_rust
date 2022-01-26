pub fn is_match(s: String, p: String) -> bool {
    let p_as_bytes = p.as_bytes();
    let s_as_bytes = s.as_bytes();

    if p_as_bytes.len() == 0 {
        return s_as_bytes.len() == 0;
    }
    let fmatch = s.len() != 0 && (p_as_bytes[0] == s_as_bytes[0] || p_as_bytes[0] == b'.');

    if p_as_bytes.len() > 1 && p_as_bytes[1] == b'*' {
        return is_match(String::from_utf8(s_as_bytes.to_vec()).unwrap(),
            String::from_utf8(p_as_bytes[2..].to_vec()).unwrap()) ||
            (fmatch && is_match(String::from_utf8(s_as_bytes[1..].to_vec()).unwrap(), p));
    } else {
        return fmatch && is_match(String::from_utf8(s_as_bytes[1..].to_vec()).unwrap(), String::from_utf8(p_as_bytes[1..].to_vec()).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match_1() {
        assert_eq!(is_match(String::from("aa"), String::from("a")), false);
    }

    #[test]
    fn test_is_match_2() {
        assert_eq!(is_match(String::from("aa"), String::from("a*")), true);
    }

    #[test]
    fn test_is_match_3() {
        assert_eq!(is_match(String::from("aa"), String::from(".*")), true);
    }

    #[test]
    fn test_is_match_4() {
        assert_eq!(is_match(String::from("aaa"), String::from("a*")), true);
    }

    #[test]
    fn test_is_match_5() {
        assert_eq!(is_match(String::from("ab"), String::from(".*")), true);
    }

    #[test]
    fn test_is_match_6() {
        assert_eq!(is_match(String::from(""), String::from(".*")), true);
    }

    #[test]
    fn test_is_match_7() {
        assert_eq!(is_match(String::from(" "), String::from(".*")), true);
    }

    #[test]
    fn test_is_match_8() {
        assert_eq!(is_match(String::from(" bc"), String::from(".*b")), false);
    }

    #[test]
    fn test_is_match_9() {
        assert_eq!(is_match(String::from("a"), String::from("a*")), true);
    }

    #[test]
    fn test_is_match_10() {
        assert_eq!(is_match(String::from("aaa"), String::from("a*a")), true);
    }

    #[test]
    fn test_is_match_11() {
        assert_eq!(is_match(String::from("aabcbcbcaccbcaabc"), String::from(".*a*aa*.*b*.c*.*a*")), true);
    }

}
