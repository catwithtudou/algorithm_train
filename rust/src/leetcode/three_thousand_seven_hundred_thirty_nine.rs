pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        cnt.insert(0, 1);

        let mut s = 0_i32;
        let mut f = 0_i32;
        let mut ans = 0_i64;

        for x in nums {
            if x == target {
                f += *cnt.get(&s).unwrap_or(&0);
                s += 1;
            } else {
                s -= 1;
                f -= *cnt.get(&s).unwrap_or(&0);
            }

            ans += f as i64;

            *cnt.entry(s).or_insert(0) += 1;
        }

        ans
    }
}