pub fn my_atoi(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut num: i128 = 0;
    let mut index: Option<i128>;
    let mut sign: char = '+';
    let mut i = 0;

    if chars.len() > 0 {
        while i < chars.len() && chars[i] == ' ' {
            i += 1;
        }
        if chars.len() == i {
            return 0;
        }
        if chars[i] == '-' || chars[i] == '+' {
            sign = chars[i];
            i += 1
        }
        while i < chars.len() {
            index = search_index(chars[i]);
            if index.is_some() {
                num = num * 10 + index.as_ref().unwrap();
            } else {
                break
            }
            i += 1;
        }
        if sign == '-' {
            num = -num;
        }

        if num > i32::MAX as i128 || num < i32::MIN as i128 {
            return if sign == '-' { i32::MIN } else { i32::MAX };
        }
    }
    num as i32
}

fn search_index(c: char) -> Option<i128> {
    let cypher: Vec<char> = String::from("0123456789").chars().collect();
    for i in 0..cypher.len() {
        if c == cypher[i] {
            return Some(i as i128)
        }
    }
    return None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_not_atoi_1() {
        assert_eq!(my_atoi(String::from("123")), 123);
    }

    #[test]
    fn test_my_not_atoi_2() {
        assert_eq!(my_atoi(String::from("words and 987")), 0);
    }

    #[test]
    fn test_my_not_atoi_3() {
        assert_eq!(my_atoi(String::from("4193 with words")), 4193);
    }

    #[test]
    fn test_my_not_atoi_4() {
        assert_eq!(my_atoi(String::from("+123")), 123);
    }

    #[test]
    fn test_my_not_atoi_5() {
        assert_eq!(my_atoi(String::from("-123")), -123);
    }

    #[test]
    fn test_my_not_atoi_6() {
        assert_eq!(my_atoi(String::from("   -42")), -42);
    }

    #[test]
    fn test_my_not_atoi_7() {
        assert_eq!(my_atoi(String::from("-91283472332")), -2147483648);
    }

    #[test]
    fn test_my_not_atoi_8() {
        assert_eq!(my_atoi(String::from("+-12")), 0);
    }

    #[test]
    fn test_my_not_atoi_9() {
        assert_eq!(my_atoi(String::from("")), 0);
    }

    #[test]
    fn test_my_not_atoi_10() {
        assert_eq!(my_atoi(String::from(" ")), 0);
    }

    #[test]
    fn test_my_not_atoi_11() {
        assert_eq!(my_atoi(String::from("9223372036854775808")), 2147483647);
    }

    #[test]
    fn test_my_not_atoi_12() {
        assert_eq!(my_atoi(String::from("18446744073709551617")), 2147483647);
    }

}

