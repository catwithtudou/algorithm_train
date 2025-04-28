pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut pre_sum = 0;
        let mut left = 0;
        let mut ans = 0;

        for (right, &v) in nums.iter().enumerate() {
            pre_sum += v as i64;

            while pre_sum * (right - left + 1) as i64 >= k {
                pre_sum -= nums[left] as i64;
                left += 1;
            }

            ans += (right - left + 1) as i64;
        }

        ans
    }
}
