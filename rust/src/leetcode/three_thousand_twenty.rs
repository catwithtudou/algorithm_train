use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        for x in nums {
            *cnt.entry(x as i64).or_insert(0) += 1;
        }

        let mut ans = (cnt.remove(&1).unwrap_or(0) - 1) | 1;


        for &x in cnt.keys(){
            let mut res =0 ;
            let mut x = x ;
            while *cnt.get(&x).unwrap_or(&0) >= 2 {
                res += 2;
                x *= x;
            }
            ans = ans.max(res+if cnt.contains_key(&x) { 1 } else { -1 });
        }

        ans as _
    }
}
