use std::collections::LinkedList;

#[allow(dead_code)]
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results: LinkedList<Vec<i32>> = LinkedList::new();
    let mut nums = nums;
    nums.sort();

    let mut previous: Option<i32> = None;
    for i in 0..nums.len() {
        if previous.is_some() && nums[i] == previous.unwrap() {
            continue;
        }
        previous = Option::from(nums[i]);

        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            if sum == 0 {
                results.push_back(vec![nums[i], nums[j], nums[k]]);
                j += 1;
                k -= 1;
                while j < k && nums[j] == nums[j - 1] {
                    j += 1;
                }
                while j < k && nums[k] == nums[k + 1] {
                    k -= 1;
                }
            } else if sum < 0 {
                j += 1;
            } else {
                k -= 1;
            }
        }
    }
    return results.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_valid() {
        let nums = vec![-1,0,1,2,-1,-4];
        assert_eq!(three_sum(nums), vec![vec![-1,-1,2],vec![-1,0,1]]);
    }

    #[test]
    fn test_three_sum_other_valid() {
        let nums = vec![-1,0,1,2,-1,-2];
        assert_eq!(three_sum(nums), vec![vec![-2,0,2],vec![-1,-1,2],vec![-1,0,1]]);
    }

    #[test]
    fn test_three_sum_empty() {
        let nums = vec![];
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(three_sum(nums), empty);
    }

    #[test]
    fn test_three_sum_zero() {
        let nums = vec![0];
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(three_sum(nums), empty);
    }
}
