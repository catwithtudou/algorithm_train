pub struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let (mut pre_cnt, mut cnt) = (0, 0);
        let mut ans = 0;
        for i in 0..nums.len() {
            cnt += 1;
            if i == nums.len() - 1 || nums[i] >= nums[i + 1] {
                ans = ans.max((cnt / 2).max(cnt.min(pre_cnt)));
                pre_cnt = cnt;
                cnt = 0;
            }
        }
        ans
    }
}
