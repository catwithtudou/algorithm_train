use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let freq: HashMap<i32, i32> = arr.iter().fold(HashMap::new(), |mut map, &x| {
            *map.entry(x).or_insert(0) += 1;
            map
        });

        let mut cnt: Vec<i32> = freq.values().cloned().collect();
        cnt.sort_by(|a, b| b.cmp(a));

        let mut res = 0;
        for (i, &c) in cnt.iter().enumerate() {
            res += c;
            if res >= (arr.len() as i32) / 2 {
                return i as i32 + 1;
            }
        }
        -1
    }
}
