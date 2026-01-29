pub struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let inf = i32::MAX / 2;
        let mut dis = [[inf; 26]; 26];

        for i in 0..26 {
            dis[i][i] = 0;
        }

        for i in 0..original.len() {
            let x = (original[i] as usize) - ('a' as usize);
            let y = (changed[i] as usize) - ('a' as usize);
            dis[x][y] = dis[x][y].min(cost[i]);
        }

        for k in 0..26 {
            for i in 0..26 {
                if dis[i][k] == inf {
                    continue;
                }
                for j in 0..26 {
                    dis[i][j] = dis[i][j].min(dis[i][k] + dis[k][j]);
                }
            }
        }

        let mut ans = 0;

        for i in 0..source.len() {
            let x = (source.as_bytes()[i] as usize) - ('a' as usize);
            let y = (target.as_bytes()[i] as usize) - ('a' as usize);
            let d = dis[x][y];
            if d == inf {
                return -1;
            }
            ans += d as i64;
        }

        ans
    }
}
