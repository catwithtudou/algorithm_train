pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let mut sorted_arr = arr.clone();
        sorted_arr.sort_unstable();

        let mut rank = HashMap::with_capacity(sorted_arr.len());

        for (i, &x) in sorted_arr.iter().enumerate() {
            if i == 0 || x != sorted_arr[i - 1] {
                rank.insert(x, rank.len() as i32 + 1);
            }
        }

        for x in &mut arr {
            *x = rank[x];
        }

        arr
    }
}