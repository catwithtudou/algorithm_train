pub struct Solution;

impl Solution {
    pub fn palindrome_path(n: i32, edges: Vec<Vec<i32>>, s: String, queries: Vec<String>) -> Vec<bool> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            g[a].push(b);
            g[b].push(a);
        }

        // 每个节点字符的位掩码
        let mut mask: Vec<i32> = s.bytes().map(|c| 1i32 << (c - b'a')).collect();

        const LOG: usize = 17; // 2^17 > 5e4
        let mut up = vec![vec![0usize; n]; LOG];
        let mut depth = vec![0usize; n];
        let mut tin = vec![0usize; n]; // dfs 序，从 1 开始
        let mut px = vec![0i32; n]; // 初始的根到 u 前缀异或

        // 迭代 dfs（避免递归太深爆栈），弹栈顺序就是先序
        let mut order = Vec::with_capacity(n);
        let mut visited = vec![false; n];
        let mut stack = vec![0usize];
        visited[0] = true;
        px[0] = mask[0];
        while let Some(u) = stack.pop() {
            order.push(u);
            tin[u] = order.len();
            for &v in &g[u] {
                if !visited[v] {
                    visited[v] = true;
                    up[0][v] = u;
                    depth[v] = depth[u] + 1;
                    px[v] = px[u] ^ mask[v];
                    stack.push(v);
                }
            }
        }

        // tout[u] = 子树里最大的 dfs 序，倒序往父节点汇总
        let mut tout = tin.clone();
        for &u in order.iter().rev() {
            if u != 0 {
                let p = up[0][u];
                tout[p] = tout[p].max(tout[u]);
            }
        }

        for k in 1..LOG {
            for u in 0..n {
                let mid = up[k - 1][u];
                up[k][u] = up[k - 1][mid];
            }
        }

        fn lca(mut u: usize, mut v: usize, up: &[Vec<usize>], depth: &[usize]) -> usize {
            if depth[u] < depth[v] {
                std::mem::swap(&mut u, &mut v);
            }
            for k in (0..up.len()).rev() {
                if depth[u] >= depth[v] + (1 << k) {
                    u = up[k][u];
                }
            }
            if u == v {
                return u;
            }
            for k in (0..up.len()).rev() {
                if up[k][u] != up[k][v] {
                    u = up[k][u];
                    v = up[k][v];
                }
            }
            up[0][u]
        }

        // 树状数组：区间异或、单点查询（异或差分）
        fn add(bit: &mut [i32], mut i: usize, d: i32) {
            while i < bit.len() {
                bit[i] ^= d;
                i += i & i.wrapping_neg();
            }
        }
        fn pre(bit: &[i32], mut i: usize) -> i32 {
            let mut r = 0;
            while i > 0 {
                r ^= bit[i];
                i -= i & i.wrapping_neg();
            }
            r
        }
        let mut bit = vec![0i32; n + 1];

        let mut ans = Vec::new();
        for q in &queries {
            let f: Vec<&str> = q.split_whitespace().collect();
            let a: usize = f[1].parse().unwrap();
            if f[0] == "update" {
                let nm = 1i32 << (f[2].as_bytes()[0] - b'a');
                let d = mask[a] ^ nm;
                if d != 0 {
                    add(&mut bit, tin[a], d); // 子树区间 [tin, tout] 整体异或 d
                    add(&mut bit, tout[a] + 1, d);
                    mask[a] = nm;
                }
            } else {
                let b: usize = f[2].parse().unwrap();
                let l = lca(a, b, &up, &depth);
                let x = (px[a] ^ pre(&bit, tin[a])) ^ (px[b] ^ pre(&bit, tin[b])) ^ mask[l];
                ans.push(x & (x - 1) == 0); // 最多一位是 1
            }
        }
        ans
    }
}