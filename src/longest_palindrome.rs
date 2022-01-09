use std::iter::Zip;
use std::iter::Rev;
use std::slice::Iter;

pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut res: Option<String> = None;

    for i in 0..chars.len() {
        let left = (0..=i).collect::<Vec<usize>>();
        let right = (i..chars.len()).collect::<Vec<usize>>();
        let radius = left.iter().rev().zip(right.iter());
        store_palindrome_from_radius(&chars, &mut res, radius);

        let left = (0..=i).collect::<Vec<usize>>();
        let right = (i + 1..chars.len()).collect::<Vec<usize>>();
        let radius = left.iter().rev().zip(right.iter());
        store_palindrome_from_radius(&chars, &mut res, radius);
    }
    res.unwrap_or(String::new())
}

fn store_palindrome_from_radius(chars: &Vec<char>, res: &mut Option<String>, radius: Zip<Rev<Iter<usize>>, Iter<usize>>) {
    for (l, r) in radius {
        if chars[*l] != chars[*r] {
            break;
        } else if res.is_none() {
            *res = Some(chars[*l..=*r].iter().collect::<String>());
        } else if res.as_ref().unwrap().len() < *r - *l + 1 {
            *res = Some(chars[*l..=*r].iter().collect::<String>());
        }
    }
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
