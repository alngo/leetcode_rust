pub fn my_atoi(s: String) -> i32 {
    let chars: Vec<char> = s.trim().chars().collect();
    let base: i32 = 10;
    let mut num: i32 = 0;
    let mut mult: i32 = 1;

    if chars.len() > 0 {
        let mut i = 0;

        if chars[i] == '-' || chars[i] == '+' {
            mult = if chars[i] == '-' { -1 } else { 1 };
            i += 1;
        }
        while i < chars.len() {
            let digit: Option<i32> = find_digits(chars[i], base);
            if digit.is_none() {
                break;
            }

            let limit: Option<i32> = find_limit(num, digit.unwrap(), mult, base);
            if limit.is_some() {
                return limit.unwrap();
            }
            num = num * base + digit.unwrap();
            i += 1;
        }
    }
    num * mult
}

fn find_digits(c: char, base: i32) -> Option<i32> {
    let cypher: Vec<char> = "0123456789abcdef".chars().collect();

    if base <= 16 {
        for i in 0..base {
            if c == cypher[i as usize] {
                return Some(i);
            }
        }
    }
    None
}

fn find_limit(num: i32, digit: i32, mult: i32, base: i32) -> Option<i32> {
    let max_divided: i32 = i32::MAX / base;
    let max_digit: i32 = i32::MAX % base;

    if num > max_divided || (num == max_divided && digit > max_digit) {
        return match mult {
            1 => Some(i32::MAX),
            _ => Some(i32::MIN)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_not_atoi_1() {
        assert_eq!(my_atoi(String::from("123")), 123);
    }

    #[test]
    fn test_my_atoi_2() {
        assert_eq!(my_atoi(String::from("words and 987")), 0);
    }

    #[test]
    fn test_my_atoi_3() {
        assert_eq!(my_atoi(String::from("4193 with words")), 4193);
    }

    #[test]
    fn test_my_atoi_4() {
        assert_eq!(my_atoi(String::from("+123")), 123);
    }

    #[test]
    fn test_my_atoi_5() {
        assert_eq!(my_atoi(String::from("-123")), -123);
    }

    #[test]
    fn test_my_atoi_6() {
        assert_eq!(my_atoi(String::from("   -42")), -42);
    }

    #[test]
    fn test_my_atoi_7() {
        assert_eq!(my_atoi(String::from("-91283472332")), -2147483648);
    }

    #[test]
    fn test_my_atoi_8() {
        assert_eq!(my_atoi(String::from("+-12")), 0);
    }

    #[test]
    fn test_my_atoi_9() {
        assert_eq!(my_atoi(String::from("")), 0);
    }

    #[test]
    fn test_my_atoi_10() {
        assert_eq!(my_atoi(String::from(" ")), 0);
    }

    #[test]
    fn test_my_atoi_11() {
        assert_eq!(my_atoi(String::from("9223372036854775808")), 2147483647);
    }

    #[test]
    fn test_my_atoi_12() {
        assert_eq!(my_atoi(String::from("18446744073709551617")), 2147483647);
    }

}

