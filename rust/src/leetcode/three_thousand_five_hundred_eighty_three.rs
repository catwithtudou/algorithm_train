pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut cnt1: HashMap<i32, i64> = HashMap::new();
        let mut cnt12: HashMap<i32, i64> = HashMap::new();
        let mut ans: i64 = 0;

        for x in nums {
            if x % 2 == 0 {
                ans += cnt12.get(&(x / 2)).copied().unwrap_or(0);
            }
            let add = cnt1.get(&(x * 2)).copied().unwrap_or(0);
            *cnt12.entry(x).or_insert(0) += add;
            *cnt1.entry(x).or_insert(0) += 1;
        }

        (ans % MOD) as i32
    }
}
