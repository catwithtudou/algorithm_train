pub struct Solution;

impl Solution {
    fn count_tree(edges: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, [i32; 2]) {
        let n = edges.len() + 1;
        let mut g = vec![Vec::new(); n];

        // 构建邻接表
        for v in edges {
            let (x, y) = (v[0], v[1]);
            g[x as usize].push(y);
            g[y as usize].push(x);
        }

        let mut cnt = [0, 0];

        fn dfs(g: &Vec<Vec<i32>>, cnt: &mut [i32; 2], x: i32, fa: i32, depth: usize) {
            cnt[depth] += 1;
            for &y in &g[x as usize] {
                if y != fa {
                    dfs(g, cnt, y, x, depth ^ 1);
                }
            }
        }

        dfs(&g, &mut cnt, 0, -1, 0);
        (g, cnt)
    }

    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let (_, cnt2) = Self::count_tree(&edges2);
        let max2 = cnt2[0].max(cnt2[1]);
        let (g, cnt1) = Self::count_tree(&edges1);
        let mut ans = vec![0; g.len()];
        fn dfs(
            g: &Vec<Vec<i32>>,
            cnt1: &[i32; 2],
            max2: i32,
            ans: &mut Vec<i32>,
            x: i32,
            fa: i32,
            depth: usize,
        ) {
            ans[x as usize] = cnt1[depth] + max2;
            for &y in &g[x as usize] {
                if y != fa {
                    dfs(g, cnt1, max2, ans, y, x, depth ^ 1);
                }
            }
        }
        dfs(&g, &cnt1, max2, &mut ans, 0, -1, 0);
        ans
    }
}
