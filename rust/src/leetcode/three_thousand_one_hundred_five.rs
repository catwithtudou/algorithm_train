pub struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 1;
        let mut i = 0;
        while i < n - 1 {
            if nums[i] == nums[i + 1] {
                i += 1;
                continue;
            }
            let inc = nums[i] < nums[i + 1];
            let start = i;
            i += 2;
            while i < n && nums[i] != nums[i - 1] && (nums[i] > nums[i - 1]) == inc {
                i += 1;
            }
            ans = ans.max(i - start);
            i -= 1;
        }
        ans as _
    }
}
