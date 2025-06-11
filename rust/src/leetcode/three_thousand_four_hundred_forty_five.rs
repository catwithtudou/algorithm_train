pub struct Solution;

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        const INF: i32 = i32::MAX / 2;
        let mut ans = -INF;
        let k = k as usize;
        let s_bytes = s.as_bytes();

        for x in 0..5 {
            for y in 0..5 {
                if x == y {
                    continue;
                }
                let mut cur_s = [0; 5];
                let mut pre_s = [0; 5];
                let mut min_s = [[INF; 2]; 2];
                let mut left = 0;

                for (i, &b) in s_bytes.iter().enumerate() {
                    cur_s[(b - b'0') as usize] += 1;
                    let r = i + 1;

                    while r - left >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        let p = &mut min_s[pre_s[x] & 1][pre_s[y] & 1];
                        *p = (*p).min((pre_s[x] - pre_s[y]) as i32);
                        pre_s[(s_bytes[left] - b'0') as usize] += 1;
                        left += 1;
                    }
                    ans = ans.max(
                        (cur_s[x] - cur_s[y]) as i32 - min_s[(cur_s[x] & 1) ^ 1][cur_s[y] & 1],
                    );
                }
            }
        }
        ans
    }
}
