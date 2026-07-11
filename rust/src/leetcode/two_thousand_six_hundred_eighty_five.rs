pub struct Solution;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut g = vec![Vec::<usize>::new(); n];

        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;

            g[x].push(y);
            g[y].push(x);
        }

        let mut vis = vec![false; n];
        let mut ans = 0;

        for i in 0..n {
            if vis[i] {
                continue;
            }

            let mut stack = vec![i];
            vis[i] = true;

            let mut nodes = 0;
            let mut edges_count = 0;

            while let Some(x) = stack.pop() {
                nodes += 1;

                // 当前节点所有邻边
                edges_count += g[x].len();

                for &y in &g[x] {
                    if !vis[y] {
                        vis[y] = true;
                        stack.push(y);
                    }
                }
            }

            // 无向图中每条边统计了两次
            edges_count /= 2;

            // 完全图边数公式
            if edges_count == nodes * (nodes - 1) / 2 {
                ans += 1;
            }
        }

        ans
    }
}