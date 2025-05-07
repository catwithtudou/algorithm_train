pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();

        let mut dis = vec![vec![i32::MAX; m]; n];
        dis[0][0] = 0;

        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0, 0))); // 小顶堆

        while let Some(Reverse((distance, i, j))) = heap.pop() {
            if i == n - 1 && j == m - 1 {
                return distance;
            }

            if distance > dis[i][j] {
                continue;
            }

            for &(dx, dy) in &dirs {
                let x = i as i32 + dx;
                let y = j as i32 + dy;

                if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 {
                    let x = x as usize;
                    let y = y as usize;
                    let new_dis = std::cmp::max(distance, move_time[x][y]) + 1;

                    if new_dis < dis[x][y] {
                        dis[x][y] = new_dis;
                        heap.push(Reverse((new_dis, x, y)));
                    }
                }
            }
        }

        -1
    }
}
