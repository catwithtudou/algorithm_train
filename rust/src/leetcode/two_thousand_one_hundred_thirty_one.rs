pub struct Solution;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut cnt = [[0; 26]; 26];
        for w in words {
            let w = w.into_bytes();
            cnt[(w[0] - b'a') as usize][(w[1] - b'a') as usize] += 1;
        }

        let (mut ans, mut odd) = (0, 0);
        for i in 0..26 {
            let c = cnt[i][i];
            ans += c - c % 2;
            odd |= c % 2;
            for j in i + 1..26 {
                ans += cnt[i][j].min(cnt[j][i]) * 2;
            }
        }

        (ans + odd) * 2
    }
}
