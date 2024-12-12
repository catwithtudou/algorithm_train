pub struct Solution;

impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let mut count = Vec::new();
        let mut ans = 0i64;
        for i in &values {
            for &v in i {
                count.push(v);
            }
        }
        count.sort();
        for i in 0..count.len() {
            ans += count[i] as i64 * (i + 1) as i64;
        }
        ans
    }
}
