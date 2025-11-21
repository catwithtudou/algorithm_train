pub struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        let mut suf_cnt = [0; 26];
        let mut suf = 0;
        for &ch in &s[1..] {
            let ch = (ch - b'a') as usize;
            suf_cnt[ch] += 1;
            suf |= 1 << ch; // 记录后缀中出现了哪些字符
        }

        let mut pre = 0;
        let mut has = [0u32; 26];

        for i in 1..n - 1 {
            let mid = (s[i] - b'a') as usize;

            suf_cnt[mid] -= 1;

            if suf_cnt[mid] == 0 {
                suf ^= 1 << mid; // 后缀中去掉当前字符
            }

            pre |= 1 << (s[i - 1] - b'a') as usize; // 记录前缀中出现了哪些字符
            has[mid] |= pre & suf; // 前缀和后缀的交集就是中间可以插入的字符
        }

        has.into_iter().map(|mask| mask.count_ones() as i32).sum()
    }
}
