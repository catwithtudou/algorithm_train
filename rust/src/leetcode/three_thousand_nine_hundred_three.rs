pub struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut suf_min = vec![0; n];
        suf_min[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            suf_min[i] = suf_min[i + 1].min(nums[i]);
        }

        let mut pre_max = 0;
        for (i, &x) in nums.iter().enumerate() {
            pre_max = pre_max.max(x);
            if pre_max - suf_min[i] <= k {
                return i as i32 ;
            }
        }

        -1 as _
    }
}
