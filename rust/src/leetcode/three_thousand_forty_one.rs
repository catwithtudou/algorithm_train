pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut dp = HashMap::new();
        let mut ans = 0;

        for x in nums {
            let increment = dp.get(&x).copied().unwrap_or(0) + 1;
            dp.insert(x + 1, increment);

            let keep = dp.get(&(x - 1)).copied().unwrap_or(0) + 1;
            dp.insert(x, keep);

            ans = ans.max(increment.max(keep));
        }

        ans
    }
}