#[allow(dead_code)]
pub fn reverse_integer(x: i32) -> i32 {
    let radix: i64 = 10;
    let mut n: i64 = if x < 0 { -x } else { x } as i64;
    let mut reversed: i64 = 0;

    while n != 0 {
        reversed = reversed * radix + n % radix;
        n /= radix;
    }

    if x < 0 {
        reversed = -reversed;
    }
    if reversed > i32::MAX as i64 || reversed < i32::MIN as i64 {
        return 0;
    }
    reversed as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_integer_1() {
        assert_eq!(reverse_integer(123), 321);
    }

    #[test]
    fn test_reverse_integer_2() {
        assert_eq!(reverse_integer(-123), -321);
    }

    #[test]
    fn test_reverse_integer_3() {
        assert_eq!(reverse_integer(120), 21);
    }

    #[test]
    fn test_reverse_integer_4() {
        assert_eq!(reverse_integer(1534236469), 0);
    }

    #[test]
    fn test_reverse_integer_5() {
        assert_eq!(reverse_integer(-2147483646), 0);
    }
}
