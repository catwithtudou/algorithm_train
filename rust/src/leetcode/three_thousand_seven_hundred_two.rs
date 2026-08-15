pub struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let (mut sum, mut xor) = (0, 0);

        for i in 0..nums.len() {
            sum += nums[i];
            xor ^= nums[i];
        }

        if sum == 0 {
            return 0;
        }

        let mut ans = nums.len() as i32;
        if xor == 0 {
            ans-=1;
        }

        ans

    }
}
