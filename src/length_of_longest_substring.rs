use std::collections::HashMap;
use std::cmp::max;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut stack: HashMap<char, usize> = HashMap::new();
    let mut longest: usize = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;

    while j < chars.len() {
        let curr_c: char = chars[j];
        if stack.contains_key(&curr_c) {
            let prev_c = *stack.get(&curr_c).unwrap();
            while i < prev_c + 1 {
                stack.remove(&chars[i]);
                i += 1;
            }
            stack.insert(curr_c, j);
            j += 1;
        } else {
            stack.insert(curr_c, j);
            j += 1;
        }
        longest = max(j - i, longest);
    }
    longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_susbstring_1() {
        let input: String = String::from("abcabcbb");
        assert_eq!(length_of_longest_substring(input), 3)
    }

    #[test]
    fn test_length_of_longest_susbstring_2() {
        let input: String = String::from("bbbbb");
        assert_eq!(length_of_longest_substring(input), 1)
    }

    #[test]
    fn test_length_of_longest_susbstring_3() {
        let input: String = String::from("pwwkew");
        assert_eq!(length_of_longest_substring(input), 3)
    }

    #[test]
    fn test_length_of_longest_susbstring_4() {
        let input: String = String::from("abcabcbbb");
        assert_eq!(length_of_longest_substring(input), 3)
    }

    #[test]
    fn test_length_of_longest_susbstring_5() {
        let input: String = String::from(" ");
        assert_eq!(length_of_longest_substring(input), 1)
    }

    #[test]
    fn test_length_of_longest_susbstring_6() {
        let input: String = String::from("");
        assert_eq!(length_of_longest_substring(input), 0)
    }
}
