pub struct Solution;

impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut suf_min = vec![0; n];

        suf_min[n - 1] = nums[n - 1];

        for i in (2..n - 1).rev() {
            suf_min[i] = suf_min[i + 1].min(nums[i]);
        }

        let mut ans = 0;
        let mut pre_max = nums[0];

        for i in 1..n - 1 {
            let x = nums[i];

            if pre_max < x && x < suf_min[i + 1] {
                ans += 2;
            } else if x > nums[i - 1] && x < nums[i + 1] {
                ans += 1;
            }
            pre_max = pre_max.max(x);
        }

        ans
    }
}
