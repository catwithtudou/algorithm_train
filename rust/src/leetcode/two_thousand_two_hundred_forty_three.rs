pub struct Solution;

impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let k = k as usize;

        while s.len() > k {
            let mut t = String::new();

            for chunk in s.as_bytes().chunks(k) {
                let sum: u32 = chunk.iter().map(|&ch| (ch - b'0') as u32).sum();
                t.push_str(&sum.to_string());
            }

            s = t;
        }

        s
    }
}
