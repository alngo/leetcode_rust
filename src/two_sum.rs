use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut entries = HashMap::new();
    for (pos, num) in nums.iter().enumerate() {
        if entries.contains_key(num) {
            return vec![*entries.get(num).unwrap() as i32, pos as i32];
        }
        entries.insert(target - num, pos);
    }
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_empty() {
        let nums = vec![];
        let empty: Vec<i32> = vec![];
        assert_eq!(two_sum(nums, 0), empty);
    }

    #[test]
    fn test_two_sum_zero() {
        let nums = vec![0];
        let empty: Vec<i32> = vec![];
        assert_eq!(two_sum(nums, 0), empty);
    }
}
