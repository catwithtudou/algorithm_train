use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut groups = vec![vec![vec![]; 7]; 7];
        for s in allowed {
            let bytes = s.as_bytes();
            let a = (bytes[0] - b'A') as usize;
            let b = (bytes[1] - b'A') as usize;
            groups[a][b].push(bytes[2]);
        }

        let n = bottom.len();
        let mut pyramid = vec![vec![0u8; n]; n];
        pyramid[n - 1] = bottom.into_bytes();

        let mut vis = HashSet::new();

        fn dfs(
            i: i32,
            j: usize,
            pyramid: &mut Vec<Vec<u8>>,
            groups: &Vec<Vec<Vec<u8>>>,
            vis: &mut HashSet<(usize, Vec<u8>)>,
        ) -> bool {
            if i < 0 {
                return true;
            }

            let row_idx = i as usize;

            if j == row_idx + 1 {
                let state = (row_idx, pyramid[row_idx][..=row_idx].to_vec());
                if vis.contains(&state) {
                    return false;
                }
                vis.insert(state);
                return dfs(i - 1, 0, pyramid, groups, vis);
            }

            let left = (pyramid[row_idx + 1][j] - b'A') as usize;
            let right = (pyramid[row_idx + 1][j + 1] - b'A') as usize;

            for &top in &groups[left][right] {
                pyramid[row_idx][j] = top;
                if dfs(i, j + 1, pyramid, groups, vis) {
                    return true;
                }
            }

            return false;
        }

        dfs(n as i32 - 2, 0, &mut pyramid, &groups, &mut vis)
    }
}
