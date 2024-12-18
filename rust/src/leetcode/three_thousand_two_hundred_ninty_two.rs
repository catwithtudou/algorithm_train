pub struct Solution;

impl Solution {
    fn calc_z(s: &[u8]) -> Vec<usize> {
        let n = s.len();
        let mut z = vec![0; n];
        z[0] = n;
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i <= r {
                z[i] = z[i - l].min(r - i + 1);
            }
            while i + z[i] < n && s[z[i]] == s[i + z[i]] {
                z[i] += 1;
                l = i;
                r = i + z[i] - 1;
            }
        }
        z
    }

    fn jump(max_jumps: &[usize]) -> i32 {
        let mut ans = 0;
        let mut cur_r = 0;
        let mut nxt_r = 0;

        for (i, &v) in max_jumps.iter().enumerate() {
            nxt_r = nxt_r.max(i + v);
            if i == cur_r {
                if i == nxt_r {
                    return -1;
                }
                cur_r = nxt_r;
                ans += 1;
            }
        }
        ans
    }

    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let mut max_jumps = vec![0; n];
        let target_bytes = target.as_bytes();

        for word in words {
            let word_bytes = word.as_bytes();
            let mut concat = Vec::with_capacity(word.len() + 1 + target.len());
            concat.extend_from_slice(word_bytes);
            concat.push(b'#');
            concat.extend_from_slice(target_bytes);

            let z = Self::calc_z(&concat);
            for (i, &z_val) in z[word.len() + 1..].iter().enumerate() {
                max_jumps[i] = max_jumps[i].max(z_val);
            }
        }

        Self::jump(&max_jumps)
    }
}
