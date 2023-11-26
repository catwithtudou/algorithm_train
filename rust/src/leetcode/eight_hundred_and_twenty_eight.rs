pub struct Solution;

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let mut d: Vec<Vec<i32>> = vec![vec![-1; 1]; 26];
        for (k, v) in s.chars().enumerate() {
            d[(v as usize) - ('A' as usize)].push(k as i32);
        }
        let mut ans = 0;
        for v in d.iter_mut() {
            v.push(s.len() as i32);
            for i in 1..v.len() - 1 {
                ans += (v[i] - v[i - 1]) * (v[i + 1] - v[i])
            }
        }
        ans as i32
    }
}