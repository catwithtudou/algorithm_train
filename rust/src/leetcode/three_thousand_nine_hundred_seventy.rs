pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn shortest_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        labels: String,
        k: i32,
    ) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let labels = labels.as_bytes();

        // 构建有向图
        let mut graph = vec![Vec::new(); n];

        for e in edges {
            let (x, y, w) = (e[0] as usize, e[1] as usize, e[2] as i64);
            graph[x].push((y, w));
        }

        // dist[x][cnt]: 到达节点 x，且连续相同标签数量为 cnt 的最短距离
        let mut dist = vec![vec![i64::MAX; k + 1]; n];
        dist[0][1] = 0;

        // 最小堆：(距离, 节点, 连续相同标签数量)
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0i64, 0usize, 1usize)));

        while let Some(Reverse((d, x, cnt))) = heap.pop() {
            if d > dist[x][cnt] {
                continue;
            }

            if x == n - 1 {
                return d as i32;
            }

            for &(y, w) in &graph[x] {
                let next_cnt = if labels[x] == labels[y] {
                    cnt + 1
                } else {
                    1
                };

                if next_cnt > k {
                    continue;
                }

                let next_dist = d + w;

                if next_dist < dist[y][next_cnt] {
                    dist[y][next_cnt] = next_dist;
                    heap.push(Reverse((next_dist, y, next_cnt)));
                }
            }
        }

        -1
    }
}