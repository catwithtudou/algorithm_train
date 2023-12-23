use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut h = BinaryHeap::from(piles); // 最大堆
        for _ in 0..k {
            let top = h.pop().unwrap();
            h.push((top + 1) / 2);
        }
        h.iter().sum()
    }
}