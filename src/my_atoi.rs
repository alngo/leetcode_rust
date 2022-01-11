pub fn my_not_atoi(s: String) -> i32 {
    return match s.parse::<i32>() {
        Ok(num)  => num,
        Err(_e) => 0,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_not_atoi() {
        assert_eq!(my_not_atoi(String::from("123")), 123);
    }
}

