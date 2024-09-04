pub struct Solution;

impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        let (mut mn,mut mx) = (nums[0] as i64,nums[0] as i64);
        for &x in &nums[1..] {
            let x = x as i64;
            let tmp = mn;
            mn=mn.min(x).min(mn*x).min(mx*x);
            mx=mx.max(x).max(tmp*x).max(mx*x);
        }
        mx
    }
}