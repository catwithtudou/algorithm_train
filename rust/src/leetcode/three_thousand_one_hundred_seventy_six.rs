pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = HashMap::new();
        let mut mx = vec![0; (k + 2) as usize];
        for &v in &nums {
            let mut f = dp.entry(v).or_insert(vec![0;(k+1)as usize]);
            for j in (0..=k as usize).rev(){
                f[j]=f[j].max(mx[j])+1;
                mx[j+1]=mx[j+1].max(f[j]);
            }
        }

        mx[(k+2) as usize]
    }
}
