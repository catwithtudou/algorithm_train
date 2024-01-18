pub struct Solution;


impl Solution {
    pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
        beans.sort_unstable();
        let mut sum = 0i64;
        let mut mx = 0i64;
        for (i, &v) in beans.iter().enumerate() {
            sum += v as i64;
            mx = mx.max((v * (beans.len() - i) as i32) as i64);
        }

        sum - mx
    }
}