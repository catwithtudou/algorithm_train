pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &x) in arr.iter().enumerate() {
            pos.entry(x).or_default().push(i);
        }

        let mut vis = vec![false; n];
        vis[0] = true;

        let mut q = vec![0usize];
        let mut ans = 0;

        loop {
            let tmp = std::mem::take(&mut q);

            for i in tmp {
                if i == n - 1 {
                    return ans;
                }

                if i + 1 < n && !vis[i + 1] {
                    vis[i + 1] = true;
                    q.push(i + 1);
                }

                if i > 0 && !vis[i - 1] {
                    vis[i - 1] = true;
                    q.push(i - 1);
                }

                let x = arr[i];

                if let Some(indices) = pos.remove(&x) {
                    for j in indices {
                        if !vis[j] {
                            vis[j] = true;
                            q.push(j);
                        }
                    }
                }
            }

            ans += 1;
        }
    }
}