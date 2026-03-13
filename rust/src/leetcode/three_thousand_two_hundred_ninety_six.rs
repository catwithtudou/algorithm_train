use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut heap: BinaryHeap<Reverse<(i64, i64, i64)>> = BinaryHeap::new();

        for t in worker_times {
            let t = t as i64;
            heap.push(Reverse((t, t, t)));
        }

        let mut ans = 0_i64;

        for _ in 0..mountain_height {
            let Reverse((mut total, mut cur, base)) = heap.pop().unwrap();
            ans = total;
            cur += base;
            total += cur;
            heap.push(Reverse((total, cur, base)));
        }

        ans
    }
}
