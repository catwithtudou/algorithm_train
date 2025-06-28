pub struct Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut idx = (0..nums.len()).collect::<Vec<_>>();
        idx.sort_unstable_by_key(|&i| -nums[i]);

        idx.truncate(k as usize);
        idx.sort_unstable();

        idx.into_iter().map(|i| nums[i]).collect()
    }
}
