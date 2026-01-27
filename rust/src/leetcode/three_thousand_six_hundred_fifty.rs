pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::vec;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut g = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let wt = e[2];
            g[x].push((y, wt));
            g[y].push((x, wt * 2));
        }

        let mut dist = vec![i32::MAX; n];
        dist[0] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0)));

        while let Some(Reverse((dx, x))) = heap.pop() {
            if dx > dist[x] {
                continue;
            }
            if x == n - 1 {
                return dx;
            }
            for &(y, wt) in &g[x] {
                let new_dy = (dx as i64) + (wt as i64);
                if new_dy < dist[y] as i64 {
                    let nd_i32 = new_dy.min(i32::MAX as i64) as i32;
                    dist[y] = nd_i32;
                    heap.push(Reverse((nd_i32, y)));
                }
            }
        }

        -1
    }
}
