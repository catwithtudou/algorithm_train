pub struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let n = nums.len();
        let mut suf_max = vec![0; n + 1];
        for i in (0..n).rev() {
            suf_max[i] = suf_max[i + 1].max(nums[i]);
        }

        let mut pre_max = 0;
        for i in 0..n {
            ans = ans.max((pre_max-nums[i]) as i64 * (suf_max[i + 1] as i64));
            pre_max = pre_max.max(nums[i]);
        }
        ans
    }
}
