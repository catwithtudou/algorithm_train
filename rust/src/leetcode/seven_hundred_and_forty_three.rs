use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize - 1;
        let mut g = vec![vec![]; n];
        for t in &times {
            g[t[0] as usize - 1].push((t[1] as usize - 1, t[2]));
        }
        let mut dis = vec![i32::MAX; n];
        dis[k] = 0;
        let mut h = BinaryHeap::new();
        h.push((0, k));
        while let Some((dx, x)) = h.pop() {
            if -dx > dis[x] {
                continue;
            }
            for &(y, d) in &g[x] {
                let new_dis = -dx + d;
                if new_dis < dis[y] {
                    dis[y] = new_dis;
                    h.push((-new_dis, y));
                }
            }
        }
        let mx = *dis.iter().max().unwrap();
        if mx < i32::MAX {
            mx
        } else {
            -1
        }
    }
}
