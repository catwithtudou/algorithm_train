use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let colors = colors.as_bytes();
        let mut graph = vec![Vec::new(); n];
        let mut deg = vec![0; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            if x == y {
                return -1;
            }
            graph[x].push(y);
            deg[y] += 1;
        }

        let mut q = VecDeque::new();
        for (i, &d) in deg.iter().enumerate() {
            if d == 0 {
                q.push_back(i);
            }
        }

        let mut f = vec![vec![0; 26]; n];
        let mut ans = 0;
        let mut visited = 0;
        while let Some(x) = q.pop_front() {
            visited += 1;
            let ch = (colors[x] - b'a') as usize;
            f[x][ch] += 1;
            ans = ans.max(f[x][ch]);
            for &y in &graph[x] {
                for i in 0..26 {
                    f[y][i] = f[y][i].max(f[x][i]);
                }
                deg[y] -= 1;
                if deg[y] == 0 {
                    q.push_back(y);
                }
            }
        }

        if visited < n {
            -1
        } else {
            ans
        }
    }
}
