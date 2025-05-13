pub struct Solution;

impl Solution {
    const MOD: i32 = 1_000_000_007;

    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut cnt = [0; 26];
        for ch in s.chars() {
            cnt[(ch as u8 - b'a') as usize] += 1;
        }

        for _ in 0..t {
            let mut nxt = [0; 26];
            nxt[0] = cnt[25];
            nxt[1] = (cnt[0] + cnt[25]) % Self::MOD;
            for j in 2..26 {
                nxt[j] = cnt[j - 1];
            }
            cnt = nxt;
        }

        let mut ans = 0;
        for &num in cnt.iter() {
            ans = (ans + num) % Self::MOD;
        }
        ans
    }
}
