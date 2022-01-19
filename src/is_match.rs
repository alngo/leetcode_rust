pub fn is_match(s: String, p: String) -> bool {
    is_match_slice(s.as_bytes(), p.as_bytes())
}

fn is_match_slice(s: &[u8], p: &[u8]) -> bool {
    match (p, s) {
        ([x, b'*', _subp @ ..], [y, subs @ ..]) if *x == b'.' || x == y => {
            is_match_slice(subs, p)
        }
        ([_, b'*', subp @ ..], _) => {
            is_match_slice(s, subp)
        }
        ([x, subp @ ..], [y, subs @ ..]) if *x == b'.' || x == y => {
            is_match_slice(subs, subp)
        }
        ([], s) => s.is_empty(),
        _ => false,
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

}
