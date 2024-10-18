pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        for i in 0..n - 2 {
            if nums[i] == 0 {
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
                ans += 1;
            }
        }

        if nums[n - 2] == 0 || nums[n - 1] == 0 {
            -1
        } else {
            ans
        }
    }
}
