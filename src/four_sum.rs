use std::collections::LinkedList;

#[allow(dead_code)]
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let lenght = nums.len();
    let mut results: LinkedList<Vec<i32>> = LinkedList::new();

    if lenght >= 4 {
        let mut nums = nums;
        nums.sort();

        let mut previous_a: Option<i32> = None;
        for a in 0..(lenght - 3) {
            if previous_a.is_some() && nums[a] == previous_a.unwrap() {
                continue;
            }
            previous_a = Some(nums[a]);

            let mut previous_b: Option<i32> = None;
            for b in (a + 1)..(lenght - 2) {
                if previous_b.is_some() && nums[b] == previous_b.unwrap() {
                    continue;
                }
                previous_b = Some(nums[b]);

                let mut c = b + 1;
                let mut d = lenght - 1;
                while c < d {
                    let sum = nums[a] + nums[b] + nums[c] + nums[d];
                    if sum < target {
                        c += 1;
                    } else if sum > target {
                        d -= 1;
                    } else {
                        let mut tmp = vec![nums[a], nums[b], nums[c], nums[d]];
                        tmp.sort();
                        results.push_back(tmp);
                        c += 1;
                        d -= 1;
                        while c < d && nums[c] == nums[c - 1] {
                            c += 1;
                        }
                        while c < d && nums[d] == nums[d + 1] {
                            d -= 1;
                        }
                    }
                }
            }
        }

    }
    return results.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum_zero_valid() {
        let nums = vec![1,0,-1,0,-2,2];
        let target = 0;
        assert_eq!(four_sum(nums, target), vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]]);
    }

    #[test]
    fn test_four_sum_eight_valid() {
        let nums = vec![2,2,2,2,2];
        let target = 8;
        assert_eq!(four_sum(nums, target), vec![vec![2,2,2,2]]);
    }

    #[test]
    fn test_four_sum_zero() {
        let nums = vec![0];
        let target = 0;
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(four_sum(nums, target), empty);
    }
}
