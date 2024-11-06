pub struct Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut ans = vec![-1; nums.len() - k + 1];
        let mut cnt = 0;
        for i in 0..nums.len() {
            if i == 0 || nums[i] == nums[i - 1] + 1 {
                cnt += 1;
            } else {
                cnt = 1;
            }
            if cnt >= k {
                ans[i - k + 1] = nums[i];
            }
        }
        ans
    }
}
