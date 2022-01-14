pub fn is_match(_s: String, _p: String) -> bool {
    return false
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
}
