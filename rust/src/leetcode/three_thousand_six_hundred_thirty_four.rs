pub struct Solution;

impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut max_save = 0;
        let mut left = 0;
        let k_i64 = k as i64;

        for (i, &mx) in nums.iter().enumerate() {
            while (nums[left] as i64) * k_i64 < mx as i64 {
                left += 1;
            }
            max_save = max_save.max((i - left + 1) as i32);
        }

        (nums.len() as i32) - max_save
    }
}
