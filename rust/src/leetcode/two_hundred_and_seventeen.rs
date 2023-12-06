use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();
        (&nums).windows(2).fold(false, |mut res, y| {
            (y[0] == y[1]) | res
        })
    }

    pub fn contains_duplicate_hash_map(mut nums: Vec<i32>) -> bool {
        let mut m: HashMap<i32, bool> = HashMap::new();
        for v in nums {
            if m.contains_key(&v) {
                return true;
            }
            m.insert(v, true);
        }
        false
    }
}