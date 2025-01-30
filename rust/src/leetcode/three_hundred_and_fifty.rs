use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut cnt = HashMap::new();
        let mut ans = vec![];
        for x in nums1 {
            *cnt.entry(x).or_insert(0) += 1;
        }
        for x in nums2 {
            if let Some(v) = cnt.get_mut(&x) {
                if *v > 0 {
                    ans.push(x);
                    *v -= 1;
                }
            }
        }

        ans
    }
}
