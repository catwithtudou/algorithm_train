pub struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let (m, n) = (m as usize, n as usize);
        let mut ans = Vec::with_capacity(m);
        ans.push(".".repeat(n));
        let row = "#".repeat(n - 1) + ".";
        for _ in 1..m {
            ans.push(row.clone());
        }
        ans
    }
}