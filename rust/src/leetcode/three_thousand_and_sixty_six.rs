use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = BinaryHeap::new();
        for x in nums {
            h.push(-x as i64);
        }
        let mut ans = 0;
        while -h.peek().unwrap() < k as i64 {
            let (x, y) = (h.pop().unwrap(), h.pop().unwrap());
            h.push((x * 2 + y) as i64);
            ans += 1;
        }
        ans
    }
}
