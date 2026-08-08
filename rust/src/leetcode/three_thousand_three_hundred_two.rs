pub struct Solution;

impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();

        let n = w1.len();
        let m = w2.len();

        // suf[i] 表示从 word1[i..] 开始，
        // 最少还需要匹配 word2 的哪个位置
        let mut suf = vec![0; n + 1];

        suf[n] = m;

        let mut j = m as i32 - 1;

        for i in (0..n).rev() {
            if j >= 0 && w1[i] == w2[j as usize] {
                j -= 1;
            }

            suf[i] = (j + 1) as usize;
        }

        let mut ans = vec![0_i32; m];

        let mut changed = false;
        let mut j = 0usize;

        for i in 0..n {
            if j == m {
                break;
            }

            if w1[i] == w2[j]
                || (!changed && suf[i + 1] <= j + 1)
            {
                if w1[i] != w2[j] {
                    changed = true;
                }

                ans[j] = i as i32;
                j += 1;

                if j == m {
                    return ans;
                }
            }
        }

        Vec::new()
    }
}