pub struct Solution;

impl Solution {
    pub fn smallest_subarrays(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![1; nums.len()];

        for i in 0..nums.len() {
            let x = nums[i];
            for j in (0..i).rev() {
                if (nums[j] | x) == nums[j] {
                    break;
                }
                nums[j] |= x;
                ans[j] = (i - j) as i32 + 1;
            }
        }

        ans
    }
}
