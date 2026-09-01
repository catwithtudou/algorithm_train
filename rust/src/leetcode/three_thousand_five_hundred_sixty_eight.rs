pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let grid: Vec<&[u8]> = classroom.iter().map(|row| row.as_bytes()).collect();

        let m = grid.len();
        let n = grid[0].len();

        let mut litter_mask = vec![vec![0usize; n]; m];

        let mut litter_count = 0usize;
        let mut start = (0usize, 0usize);

        // 给每个 L 分配一个 bit
        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    b'L' => {
                        litter_mask[i][j] = 1usize << litter_count;
                        litter_count += 1;
                    }
                    b'S' => {
                        start = (i, j);
                    }
                    _ => {}
                }
            }
        }

        let target_mask = (1usize << litter_count) - 1;

        let dirs = [
            (-1isize, 0isize),
            (1, 0),
            (0, -1),
            (0, 1),
        ];

        // (x, y, energy_left, mask, moves)
        let mut queue = VecDeque::new();
        queue.push_back((start.0, start.1, energy, 0usize, 0i32));

        let mut visited = HashSet::new();
        visited.insert((start.0, start.1, energy, 0usize));

        while let Some((x, y, e, mask, moves)) = queue.pop_front() {
            if mask == target_mask {
                return moves;
            }

            // 没能量就不能再移动
            if e == 0 {
                continue;
            }

            for &(dx, dy) in &dirs {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0
                    || nx >= m as isize
                    || ny < 0
                    || ny >= n as isize
                {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] == b'X' {
                    continue;
                }

                // 移动消耗 1 点能量
                let mut new_energy = e - 1;

                // 到达 R 时恢复满能量
                if grid[nx][ny] == b'R' {
                    new_energy = energy;
                }

                // 如果是 L，则记录已经收集
                let new_mask = mask | litter_mask[nx][ny];

                let state = (nx, ny, new_energy, new_mask);

                if visited.insert(state) {
                    queue.push_back((
                        nx,
                        ny,
                        new_energy,
                        new_mask,
                        moves + 1,
                    ));
                }
            }
        }

        -1
    }
}