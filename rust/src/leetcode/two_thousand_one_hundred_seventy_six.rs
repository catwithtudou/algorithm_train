use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut ans = 0;
        for i in 0..nums.len() {
            let v = cnt.entry(nums[i]).or_insert(Vec::new());
            for j in 0..v.len() {
                if (i * v[j]) as i32 % k == 0 {
                    ans += 1;
                }
            }
            v.push(i);
        }
        ans
    }
}
