pub fn zigzag_conversion(s: String, num_rows: i32) -> String {
    let chars: Vec<char> = s.chars().collect();
    let step: usize = calculate_step(num_rows);
    let mut result: Vec<char> = Vec::new();

    for i in 0..num_rows as usize {
        let mut j: usize = i;
        while j < chars.len() {
            result.push(chars[j]);
            if i != 0 && i != num_rows as usize - 1 && (j + step - 2 * i) < chars.len() {
                result.push(chars[j + step - 2 * i])
            }
            j += step;
        }
    }

    result.iter().collect::<String>()
}

pub fn calculate_step(num_rows: i32) -> usize {
    let step: usize = (2 * num_rows - 2) as usize;
    if step == 0 {
        return 1;
    }
    step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zigzag_conversion_1() {
        assert_eq!(zigzag_conversion(String::from("PAYPALISHIRING"), 3), String::from("PAHNAPLSIIGYIR"));
    }

    #[test]
    fn test_zigzag_conversion_2() {
        assert_eq!(zigzag_conversion(String::from("PAYPALISHIRING"), 4), String::from("PINALSIGYAHRPI"));
    }

    #[test]
    fn test_zigzag_conversion_3() {
        assert_eq!(zigzag_conversion(String::from("PAYPALISHIRING"), 5), String::from("PHASIYIRPLIGAN"));
    }

    #[test]
    fn test_zigzag_conversion_4() {
        assert_eq!(zigzag_conversion(String::from("A"), 1), String::from("A"));
    }
}
