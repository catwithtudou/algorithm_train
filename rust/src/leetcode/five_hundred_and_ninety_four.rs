pub struct Solution;

impl Solution {
    pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        nums.sort_unstable();
        let mut left = 0;
        for (i, &v) in nums.iter().enumerate() {
            while v - nums[left] > 1 {
                left += 1;
            }

            if nums[i] - nums[left] == 1 {
                ans = ans.max((i - left + 1) as i32)
            }
        }

        ans
    }
}
