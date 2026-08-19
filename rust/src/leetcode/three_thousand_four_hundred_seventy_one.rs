pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;

        if k == n {
            return *nums.iter().max().unwrap();
        }

        if k == 1 {
            let mut freq = HashMap::new();

            for &x in &nums {
                *freq.entry(x).or_insert(0) += 1;
            }

            return freq
                .into_iter()
                .filter(|&(_, count)| count == 1)
                .map(|(x, _)| x)
                .max()
                .unwrap_or(-1);
        }

        let check = |slice: &[i32], x: i32| {
            if slice.contains(&x) {
                -1
            } else {
                x
            }
        };

        check(&nums[1..], nums[0])
            .max(check(&nums[..n - 1], nums[n - 1]))
    }
}