pub struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, mut walls: Vec<i32>) -> i32 {
        let n = robots.len();

        let mut a = vec![(0_i32, 0_i32); n + 2];
        for i in 0..n {
            a[i] = (robots[i], distance[i]);
        }
        a[n + 1] = (i32::MAX, 0); // 哨兵

        a.sort_unstable_by_key(|&(x, _)| x);
        walls.sort_unstable();

        let mut f = vec![[0_i32; 2]; n + 1];

        for i in 1..=n {
            let (x, d) = a[i];

            // 往左射，墙的坐标范围为 [left_x, x]
            let left_x = (x - d).max(a[i - 1].0 + 1);
            let left = lower_bound(&walls, left_x);
            let mut cur = lower_bound(&walls, x + 1);
            let left_res = f[i - 1][0] + (cur - left) as i32;

            cur = lower_bound(&walls, x);
            for j in 0..2 {
                // 往右射，墙的坐标范围为 [x, right_x]
                let mut x2 = a[i + 1].0;
                if j == 0 {
                    // 右边那个机器人往左射
                    x2 -= a[i + 1].1;
                }
                let right_x = (x + d).min(x2 - 1);
                let right = lower_bound(&walls, right_x + 1);
                f[i][j] = left_res.max(f[i - 1][1] + (right - cur) as i32);
            }
        }

        f[n][1]
    }
}

fn lower_bound(nums: &[i32], target: i32) -> usize {
    nums.partition_point(|&x| x < target)
}
