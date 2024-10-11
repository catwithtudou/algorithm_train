pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut cnt1 = HashMap::new();
        for x in nums1 {
            if x % k == 0 {
                *cnt1.entry(x / k).or_insert(0) += 1;
            }
        }
        if cnt1.is_empty() {
            return 0;
        }
        let mut cnt2 = HashMap::new();
        for x in nums2 {
            *cnt2.entry(x).or_insert(0) += 1;
        }

        let mut ans = 0i64;
        let u = *cnt1.keys().max().unwrap();
        for (x, cnt) in cnt2 {
            let mut s = 0;
            for y in (x..=u).step_by(x as usize) {
                if let Some(&c) = cnt1.get(&y) {
                    s += c;
                }
            }
            ans += s as i64 * cnt as i64;
        }

        ans
    }
}
