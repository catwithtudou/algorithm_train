pub struct Solution;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut edges = vec![Vec::<(usize, i32)>::new(); n + 1];

        for road in roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let dis = road[2];

            edges[u].push((v, dis));
            edges[v].push((u, dis));
        }

        let mut ans = i32::MAX;
        let mut vis = vec![false; n + 1];

        fn dfs(
            x: usize,
            edges: &Vec<Vec<(usize, i32)>>,
            vis: &mut Vec<bool>,
            ans: &mut i32,
        ) {
            vis[x] = true;

            for &(to, dis) in &edges[x] {
                *ans = (*ans).min(dis);

                if !vis[to] {
                    dfs(to, edges, vis, ans);
                }
            }
        }

        dfs(1, &edges, &mut vis, &mut ans);

        ans
    }
}