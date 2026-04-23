pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut groups: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &x) in nums.iter().enumerate() {
            groups.entry(x).or_default().push(i);
        }

        let mut ans = vec![0_i64; nums.len()];

        for positions in groups.values() {
            let n = positions.len();
            let mut prefix_sum = vec![0_i64; n + 1];

            for (i, &pos) in positions.iter().enumerate() {
                prefix_sum[i + 1] = prefix_sum[i] + pos as i64;
            }

            for (i, &target) in positions.iter().enumerate() {
                let target = target as i64;
                let i_i64 = i as i64;
                let n_i64 = n as i64;

                let left = target * i_i64 - prefix_sum[i];
                let right = (prefix_sum[n] - prefix_sum[i]) - target * (n_i64 - i_i64);

                ans[target as usize] = left + right;
            }
        }

        ans
    }
}