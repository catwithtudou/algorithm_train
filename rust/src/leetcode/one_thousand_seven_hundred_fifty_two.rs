pub struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut sorted = true;
        for i in 1..n {
            if nums[i - 1] > nums[i] {
                if !sorted {
                    return false;
                }
                sorted = false;
            }
        }
        sorted || nums[0] >= nums[n - 1]
    }
}
