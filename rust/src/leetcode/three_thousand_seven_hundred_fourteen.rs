pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut ans = 0;

        let mut i = 0;
        while i < n {
            let start = i;
            i += 1;
            while i < n && s[i] == s[i - 1] {
                i += 1;
            }
            ans = ans.max(i - start);
        }

        let  two = |x: u8, y: u8, ans: &mut usize| {
            let mut i = 0;
            while i < n {
                // 仅处理包含 x 或 y 的连续段
                let mut pod = HashMap::new();
                pod.insert(0, (i as i32) - 1);
                let mut d = 0;

                while i < n && (s[i] == x || s[i] == y) {
                    if s[i] == x { d += 1; } else { d -= 1; }

                    if let Some(&j) = pod.get(&d) {
                        *ans = (*ans).max(i - j as usize);
                    } else {
                        pod.insert(d, i as i32);
                    }
                    i += 1;
                }
                i += 1;
            }
        };

        two(b'a', b'b', &mut ans);
        two(b'a', b'c', &mut ans);
        two(b'b', b'c', &mut ans);


        let mut pos = HashMap::new();
        pos.insert((0, 0), -1_i32);
        let mut cnt = [0; 3];

        for (idx, &b) in s.iter().enumerate() {
            cnt[(b - b'a') as usize] += 1;
            let p = (cnt[0] - cnt[1], cnt[1] - cnt[2]);

            if let Some(&j) = pos.get(&p) {
                ans = ans.max(idx - j as usize);
            } else {
                pos.insert(p, idx as i32);
            }
        }

        ans as i32
    }
}