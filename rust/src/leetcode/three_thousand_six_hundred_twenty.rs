pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        #[derive(Clone, Copy)]
        struct Edge {
            to: usize,
            wt: i64,
        }

        let n = online.len();
        let mut g = vec![Vec::<Edge>::new(); n];
        let mut deg = vec![0_i32; n];

        let mut max_wt = -1_i32;

        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let wt = e[2];

            if online[x] && online[y] {
                g[x].push(Edge {
                    to: y,
                    wt: wt as i64,
                });

                deg[y] += 1;

                if x == 0 {
                    max_wt = max_wt.max(wt);
                }
            }
        }

        // 先清理无法从 0 到达的点/边
        let mut q = VecDeque::new();

        for i in 1..n {
            if deg[i] == 0 {
                q.push_back(i);
            }
        }

        while let Some(x) = q.pop_front() {
            for e in &g[x] {
                let y = e.to;
                deg[y] -= 1;

                if deg[y] == 0 && y > 0 {
                    q.push_back(y);
                }
            }
        }

        // check(lower) == true 表示：
        // 如果只允许走 wt >= lower 的边，那么无法在总权重 <= k 内到达终点
        let check = |lower: i64| -> bool {
            let mut deg = deg.clone();

            let inf = i64::MAX / 4;
            let mut f = vec![inf; n];
            f[0] = 0;

            let mut q = VecDeque::new();
            q.push_back(0usize);

            while let Some(x) = q.pop_front() {
                if x == n - 1 {
                    return f[x] > k;
                }

                for e in &g[x] {
                    let y = e.to;
                    let wt = e.wt;

                    if wt >= lower {
                        f[y] = f[y].min(f[x] + wt);
                    }

                    deg[y] -= 1;

                    if deg[y] == 0 {
                        q.push_back(y);
                    }
                }
            }

            true
        };

        // 等价 sort.Search(maxWt+1, check) - 1
        let mut left = 0_i64;
        let mut right = (max_wt + 1) as i64;

        while left < right {
            let mid = (left + right) / 2;

            if check(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        (left - 1) as i32
    }
}
