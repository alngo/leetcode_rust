pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums: Vec<i32> = nums1;
    let mut n2: Vec<i32> = nums2;
    nums.append(&mut n2);
    nums.sort();
    let len: usize = nums.len();
    let mid: usize = len / 2;

    if len % 2 == 0 {
        return (nums[mid] + nums[mid - 1]) as f64 / 2.0;
    } else {
        return nums[mid] as f64;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays_1() {
        assert_eq!(find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
    }

    #[test]
    fn test_find_median_sorted_arrays_2() {
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
    }
}
