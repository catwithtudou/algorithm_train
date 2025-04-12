use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let n = n as usize;

        let mut factorial = vec![1; n + 1];
        for i in 1..=n {
            factorial[i] = factorial[i - 1] * i;
        }

        let mut ans: i64 = 0;
        let mut vis = HashMap::new();

        let base = 10_i32.pow(((n - 1) / 2) as u32) as i32;
        for i in base..base * 10 {
            let (mut x, mut t) = (i, i);

            if n % 2 > 0 {
                t /= 10;
            }

            while t > 0 {
                x = x * 10 + t % 10;
                t /= 10;
            }

            if x % k > 0 {
                continue;
            }

            let x_str = x.to_string();
            let mut bs: Vec<char> = x_str.chars().collect();
            bs.sort();
            let s: String = bs.iter().collect();

            if vis.contains_key(&s) {
                continue;
            }
            vis.insert(s.clone(), true);

            let mut cnt = [0; 10];
            for c in bs {
                cnt[(c as u8 - b'0') as usize] += 1;
            }

            let mut res = (n - cnt[0]) * factorial[n - 1];
            for c in cnt.iter() {
                res /= factorial[*c];
            }
            ans += res as i64;
        }
        ans
    }
}
