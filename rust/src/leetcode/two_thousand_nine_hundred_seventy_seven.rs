pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        const INF: i64 = 1_000_000_000_000_000_000; // 1e18

        // ---------- Trie ----------
        #[derive(Clone)]
        struct Node {
            son: [Option<usize>; 26],
            sid: Option<usize>, // None == -1
        }
        impl Node {
            fn new() -> Self {
                Self {
                    son: [None; 26],
                    sid: None,
                }
            }
        }

        let mut nodes: Vec<Node> = vec![Node::new()]; // root = 0
        let mut next_sid: usize = 0;

        fn put(s: &str, nodes: &mut Vec<Node>, next_sid: &mut usize) -> usize {
            let mut o = 0usize;
            for &b in s.as_bytes() {
                let idx = (b - b'a') as usize;
                if nodes[o].son[idx].is_none() {
                    nodes.push(Node::new());
                    let ni = nodes.len() - 1;
                    nodes[o].son[idx] = Some(ni);
                }
                o = nodes[o].son[idx].unwrap();
            }
            if nodes[o].sid.is_none() {
                nodes[o].sid = Some(*next_sid);
                *next_sid += 1;
            }
            nodes[o].sid.unwrap()
        }

        let m = cost.len();
        let cap = m * 2;
        let mut dis = vec![vec![INF; cap]; cap];
        for i in 0..cap {
            dis[i][i] = 0;
        }

        // build edges: dis[sid1][sid2] = min(dis[sid1][sid2], c)
        for i in 0..m {
            let sid1 = put(&original[i], &mut nodes, &mut next_sid);
            let sid2 = put(&changed[i], &mut nodes, &mut next_sid);
            dis[sid1][sid2] = min(dis[sid1][sid2], cost[i] as i64);
        }

        // ---------- Floyd on [0..sid) ----------
        let sid = next_sid;
        for k in 0..sid {
            for i in 0..sid {
                if dis[i][k] >= INF {
                    continue;
                }
                for j in 0..sid {
                    if dis[k][j] >= INF {
                        continue;
                    }
                    let cand = dis[i][k] + dis[k][j];
                    if cand < dis[i][j] {
                        dis[i][j] = cand;
                    }
                }
            }
        }

        // ---------- DFS + memo ----------
        let s = source.into_bytes();
        let t = target.into_bytes();
        let n = s.len();

        let mut memo: Vec<i64> = vec![-1; n + 1];
        memo[n] = 0;

        fn dfs(
            i: usize,
            n: usize,
            s: &[u8],
            t: &[u8],
            nodes: &Vec<Node>,
            dis: &Vec<Vec<i64>>,
            memo: &mut Vec<i64>,
        ) -> i64 {
            const INF: i64 = 1_000_000_000_000_000_000;

            if i >= n {
                return 0;
            }
            if memo[i] != -1 {
                return memo[i];
            }

            let mut res = INF;

            // if source[i] == target[i], can skip 1 char
            if s[i] == t[i] {
                res = dfs(i + 1, n, s, t, nodes, dis, memo);
            }

            // try match substrings in trie simultaneously
            let mut p = 0usize; // root
            let mut q = 0usize; // root

            for j in i..n {
                let ps = (s[j] - b'a') as usize;
                let qt = (t[j] - b'a') as usize;

                match nodes[p].son[ps] {
                    Some(nx) => p = nx,
                    None => break,
                }
                match nodes[q].son[qt] {
                    Some(nx) => q = nx,
                    None => break,
                }

                if let (Some(sid1), Some(sid2)) = (nodes[p].sid, nodes[q].sid) {
                    let d = dis[sid1][sid2];
                    if d >= INF {
                        continue;
                    }
                    let tail = dfs(j + 1, n, s, t, nodes, dis, memo);
                    if tail >= INF {
                        continue;
                    }
                    res = min(res, tail + d);
                }
            }

            memo[i] = res;
            res
        }

        let ans = dfs(0, n, &s, &t, &nodes, &dis, &mut memo);
        if ans >= INF {
            -1
        } else {
            ans
        }
    }
}
