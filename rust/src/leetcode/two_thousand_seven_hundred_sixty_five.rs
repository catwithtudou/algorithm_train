pub struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        let mut i = 0;
        let n = nums.len();
        while i < n - 1 {
            if nums[i + 1] - nums[i] != 1 {
                i += 1;
                continue;
            }
            let i0 = i;
            i += 2;
            while i < n && nums[i - 2] == nums[i] {
                i += 1;
            }
            ans = ans.max((i - i0) as i32);
            i -= 1;
        }


        ans
    }
}