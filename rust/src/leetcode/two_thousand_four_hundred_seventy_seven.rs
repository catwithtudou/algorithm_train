pub struct Solution;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let mut graph = vec![vec![]; roads.len() + 1];
        for e in &roads {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            graph[y].push(x);
        }

        let mut ans = 0i64;
        Self::dfs(0, 0, &graph, seats, &mut ans);
        ans
    }

    fn dfs(x: usize, fa: usize, g: &Vec<Vec<usize>>, seats: i32, ans: &mut i64) -> i32 {
        let mut size = 1;
        for &y in &g[x] {
            if y != fa {
                size += Self::dfs(y, x, g, seats, ans);
            }
        }
        if x != 0 {
            *ans += ((size - 1) / seats + 1) as i64;
        }
        size
    }
}