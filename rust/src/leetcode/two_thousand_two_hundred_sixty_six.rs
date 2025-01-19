pub struct Solution;

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let m = 1000000007;
        let mut f = vec![1, 1, 2, 4];
        let mut g = vec![1, 1, 2, 4];

        for i in 4..=pressed_keys.len() {
            f.push((f[i - 1] + f[i - 2] + f[i - 3]) % m);
            g.push((g[i - 1] + g[i - 2] + g[i - 3] + g[i - 4]) % m);
        }

        let (mut ans, mut cnt) = (1, 0);
        let pressed_keys: Vec<char> = pressed_keys.chars().collect();

        for i in 0..pressed_keys.len() {
            cnt += 1;
            if i == pressed_keys.len() - 1 || pressed_keys[i + 1] != pressed_keys[i] {
                if pressed_keys[i] != '7' && pressed_keys[i] != '9' {
                    ans = (ans * f[cnt]) % m as i64;
                } else {
                    ans = (ans * g[cnt]) % m as i64;
                }
                cnt = 0;
            }
        }

        ans as i32
    }
}
