pub struct Solution;

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let n = parent.len();
        let s = s.as_bytes();

        let mut g = vec![Vec::<usize>::new(); n];
        for (i, &p) in parent.iter().enumerate().skip(1) {
            g[p as usize].push(i);
        }

        let mut anc = [None; 26];

        fn rebuild(
            x: usize,
            s: &[u8],
            g: &mut [Vec<usize>],
            anc: &mut [Option<usize>; 26],
        ) {
            // ① 递归开始：进入节点 x
            let sx = (s[x] - b'a') as usize;
            let old = anc[sx];
            anc[sx] = Some(x);

            // 只遍历当前节点原有的子节点
            let len = g[x].len();

            for i in 0..len {
                let y = g[x][i];

                if let Some(a) = anc[(s[y] - b'a') as usize] {
                    g[a].push(y);
                    g[x][i] = usize::MAX; // 标记删除
                }

                // ② 开始对子节点 y 递归
                rebuild(y, s, g, anc);
                // ③ 子节点 y 递归结束
            }

            // ④ 恢复现场，当前节点 x 递归结束
            anc[sx] = old;
        }

        // 开始重建树
        rebuild(0, s, &mut g, &mut anc);

        let mut size = vec![0; n];

        fn dfs(x: usize, g: &[Vec<usize>], size: &mut [i32]) {
            // ① 递归开始
            size[x] = 1;

            for &y in &g[x] {
                if y != usize::MAX {
                    // ② 开始对子节点 y 递归
                    dfs(y, g, size);
                    // ③ 子节点 y 递归结束

                    size[x] += size[y];
                }
            }

            // ④ 当前节点 x 递归结束
        }

        // 计算子树大小
        dfs(0, &g, &mut size);

        size
    }
}