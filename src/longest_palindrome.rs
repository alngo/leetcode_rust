pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut res: Option<String> = None;

    for i in 0..chars.len() {
        // for odd length palindrome
        let left = (0..=i).collect::<Vec<usize>>();
        let right = (i..chars.len()).collect::<Vec<usize>>();
        let radius = left.iter().rev().zip(right.iter());
        for (l, r) in radius {
            if chars[*l] != chars[*r] {
                break;
            }
            if res.is_none() {
                res = Some(chars[*l..=*r].iter().collect::<String>());
            }
            if res.as_ref().unwrap().len() < *r - *l + 1 {
                res = Some(chars[*l..=*r].iter().collect::<String>());
            }
        }

        // for even length palindrome
        let left = (0..=i).collect::<Vec<usize>>();
        let right = (i + 1..chars.len()).collect::<Vec<usize>>();
        let radius = left.iter().rev().zip(right.iter());
        for (l, r) in radius {
            if chars[*l] != chars[*r] {
                break;
            }
            if res.is_none() {
                res = Some(chars[*l..=*r].iter().collect::<String>());
            }
            if res.as_ref().unwrap().len() < *r - *l + 1 {
                res = Some(chars[*l..=*r].iter().collect::<String>());
            }
        }
    }
    res.unwrap_or(String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome_1() {
        assert_eq!(longest_palindrome(String::from("babad")), String::from("bab"));
    }

    #[test]
    fn test_longest_palindrome_2() {
        assert_eq!(longest_palindrome(String::from("cbbd")), String::from("bb"));
    }

    #[test]
    fn test_longest_palindrome_3() {
        assert_eq!(longest_palindrome(String::from("babaddda")), String::from("addda"));
    }

    #[test]
    fn test_longest_palindrome_5() {
        assert_eq!(longest_palindrome(String::from("babab")), String::from("babab"));
    }

    #[test]
    fn test_longest_palindrome_6() {
        assert_eq!(longest_palindrome(String::from("a")), String::from("a"));
    }
}
