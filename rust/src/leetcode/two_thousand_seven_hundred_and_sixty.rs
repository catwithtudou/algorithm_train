pub struct Solution;

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        let mut i = 0;
        while i < n {
            if nums[i] % 2 != 0 || nums[i] > threshold {
                i += 1;
                continue;
            }
            let start = i;
            i += 1;
            while i < n && nums[i] <= threshold && nums[i] % 2 != nums[i - 1] % 2 {
                i += 1;
            }
            ans = ans.max(i - start);
        }
        ans as i32
    }
}