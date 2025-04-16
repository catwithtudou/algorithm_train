use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut ans = 0;
        let (mut left, mut pairs) = (0, 0);
        let mut cnt = HashMap::new();
        for &v in &nums {
            let e = cnt.entry(v).or_insert(0);
            pairs += *e;
            *e += 1;
            while pairs >= k {
                let e = cnt.entry(nums[left]).or_insert(0);
                *e -= 1;
                pairs -= *e;
                left += 1;
            }
            ans += left as i64;
        }

        ans
    }
}
