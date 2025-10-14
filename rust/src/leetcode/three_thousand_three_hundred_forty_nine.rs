pub struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let (mut pre_cnt, mut cnt) = (0, 0);
        let (mut max_k) = 0;
        for i in 0..nums.len() {
            cnt += 1;
            if i == nums.len() - 1 || nums[i] >= nums[i + 1] {
                max_k = max_k.max((cnt / 2).max(cnt.min(pre_cnt)));
                pre_cnt = cnt;
                cnt = 0;
            }
        }
        k <= max_k
    }
}
