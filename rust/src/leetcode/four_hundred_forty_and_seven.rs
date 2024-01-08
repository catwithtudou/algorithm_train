use std::collections::HashMap;

pub struct Solution;


impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut cnt = HashMap::new();
        for p in &points {
            cnt.clear();
            for q in &points {
                let dis = (p[1] - q[1]) * (p[1] - q[1]) + (p[0] - q[0]) * (p[0] - q[0]);
                *cnt.entry(dis).or_insert(0) += 1;
            }
            for (_, m) in cnt.iter() {
                ans += m * (m - 1);
            }
        }
        ans
    }
}