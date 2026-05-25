pub struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;

        let mut diff = vec![0; n + 1];

        // 一开始在起点 0，把 [0, 0] 加一
        diff[0] = 1;
        diff[1] = -1;

        let mut sum_d = 0;

        for i in 0..n {
            sum_d += diff[i];

            // 当前下标 i 可以到达，并且 s[i] == '0'
            if sum_d > 0 && bytes[i] == b'0' {
                // [i + min_jump, i + max_jump] 都可以到达
                let l = (i + min_jump).min(n);
                let r = (i + max_jump + 1).min(n);

                diff[l] += 1;
                diff[r] -= 1;
            }
        }

        sum_d > 0 && bytes[n - 1] == b'0'
    }
}