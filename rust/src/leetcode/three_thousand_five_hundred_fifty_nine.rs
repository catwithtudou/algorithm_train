pub struct Solution;

const MOD: i64 = 1_000_000_007;
const NONE: usize = usize::MAX;

impl Solution {
    fn lift(mut x: usize, mut diff: usize, pa: &[Vec<usize>]) -> usize {
        while diff > 0 {
            let b = diff.trailing_zeros() as usize;
            x = pa[x][b];
            diff &= diff - 1;
        }
        x
    }

    fn lca(mut x: usize, mut y: usize, dep: &[usize], pa: &[Vec<usize>]) -> usize {
        if dep[x] > dep[y] {
            std::mem::swap(&mut x, &mut y);
        }

        // 先把 y 提到和 x 同一深度
        y = Self::lift(y, dep[y] - dep[x], pa);

        if x == y {
            return x;
        }

        // 从大步长到小步长，同时往上跳
        for i in (0..pa[0].len()).rev() {
            if pa[x][i] != pa[y][i] {
                x = pa[x][i];
                y = pa[y][i];
            }
        }

        pa[x][0]
    }

    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;

        let mut g = vec![Vec::<usize>::new(); n];

        for e in edges {
            let x = e[0] as usize - 1;
            let y = e[1] as usize - 1;

            g[x].push(y);
            g[y].push(x);
        }

        // 预处理 2 的幂
        let mut pow2 = vec![1_i64; n + 1];
        for i in 1..=n {
            pow2[i] = pow2[i - 1] * 2 % MOD;
        }

        // lg = bits.Len(n)
        let lg = (usize::BITS - n.leading_zeros()) as usize;

        // pa[x][i] 表示 x 的第 2^i 个祖先
        let mut pa = vec![vec![NONE; lg]; n];
        let mut dep = vec![0usize; n];

        // 迭代 DFS，避免 Rust 递归爆栈
        let mut stack = vec![(0usize, NONE)];

        while let Some((x, parent)) = stack.pop() {
            for &y in &g[x] {
                if y != parent {
                    pa[y][0] = x;
                    dep[y] = dep[x] + 1;
                    stack.push((y, x));
                }
            }
        }

        // 倍增预处理
        for i in 0..lg - 1 {
            for x in 0..n {
                let p = pa[x][i];

                if p != NONE {
                    pa[x][i + 1] = pa[p][i];
                }
            }
        }

        let mut ans = Vec::with_capacity(queries.len());

        for q in queries {
            if q[0] == q[1] {
                ans.push(0);
                continue;
            }

            let x = q[0] as usize - 1;
            let y = q[1] as usize - 1;

            let z = Self::lca(x, y, &dep, &pa);

            let dis = dep[x] + dep[y] - dep[z] * 2;

            ans.push(pow2[dis - 1] as i32);
        }

        ans
    }
}