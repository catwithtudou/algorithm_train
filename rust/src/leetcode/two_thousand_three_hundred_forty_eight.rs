pub struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut last = -1;
        for (i, x) in nums.into_iter().enumerate() {
            let i = i as i64;
            if x != 0 {
                last = i;
            } else {
                ans += (i - last) as i64;
            }
        }
        ans
    }
}
