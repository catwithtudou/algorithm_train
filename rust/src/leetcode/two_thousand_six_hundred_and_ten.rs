pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cnt = HashMap::new();

        for x in nums {
            *cnt.entry(x).or_insert(0) += 1;
        }

        let mut ans = vec![];
        while !cnt.is_empty() {
            let row = cnt.keys().cloned().collect::<Vec<_>>();
            ans.push(row.clone());

            for x in row {
                let c = cnt.get_mut(&x).unwrap();
                *c -= 1;
                if *c == 0 {
                    cnt.remove(&x);
                }
            }
        }
        ans
    }
}
