use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut seen = HashMap::new();
        let mut x = 1 % k;

        while x > 0 && !seen.contains_key(&x) {
            seen.insert(x, true);

            x = (x * 10 + 1) % k;
        }

        if x > 0 {
            return -1;
        }

        (seen.len() + 1) as _
    }
}
