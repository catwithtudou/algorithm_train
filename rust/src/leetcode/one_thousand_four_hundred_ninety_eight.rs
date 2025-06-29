pub struct Solution;

impl Solution {
    const MOD: i32 = 1_000_000_007;

    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut pow2 = vec![1i32; nums.len()];
        for i in 1..nums.len() {
            pow2[i] = (pow2[i - 1] * 2) % Self::MOD;
        }

        nums.sort_unstable();
        let mut ans = 0;
        let (mut left, mut right) = (0 as i32, nums.len() as i32 - 1);

        while left <= right {
            if nums[left as usize] + nums[right as usize] <= target {
                ans = (ans + pow2[(right - left) as usize]) % Self::MOD;
                left += 1;
            } else {
                right -= 1;
            }
        }

        ans
    }
}
