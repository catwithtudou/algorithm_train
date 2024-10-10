use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut cnt = HashMap::new();

        for mut x in nums1 {
            if x % k != 0 {
                continue;
            }
            x /= k;
            let mut d = 1;
            while d * d <= x {
                if x % d != 0 {
                    d += 1;
                    continue;
                }
                *cnt.entry(d).or_insert(0) += 1;
                if d * d < x {
                    *cnt.entry(x / d).or_insert(0) += 1;
                }
                d += 1;
            }
        }

        nums2.iter().map(|x| *cnt.get(x).unwrap_or(&0)).sum()
    }
}
