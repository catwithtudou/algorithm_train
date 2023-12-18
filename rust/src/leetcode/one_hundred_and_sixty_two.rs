pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut left, mut right) = (0, n - 1);
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] > nums[mid + 1] {
                right = mid
            } else {
                left = mid + 1
            }
        }

        left as i32
    }
}
