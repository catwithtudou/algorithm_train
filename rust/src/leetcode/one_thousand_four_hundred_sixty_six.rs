pub struct Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
        for edge in &connections {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            graph[a].push((b as i32, 1));
            graph[b].push((a as i32, 0));
        }
        fn dfs(x: usize, fa: i32, graph: &Vec<Vec<(i32, i32)>>) -> i32 {
            let mut ans = 0;
            for edge in &graph[x] {
                if edge.0 != fa {
                    ans += edge.1 + dfs(edge.0 as usize, x as i32, graph);
                }
            }
            ans
        }
        dfs(0, -1, &graph)
    }
}