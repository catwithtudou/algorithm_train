pub struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut f = vec![0 as i64; n + 1];

        for i in (0..n).rev() {
            f[i] = f[i+1].max(f[n.min(i+questions[i][1] as usize+1)]+questions[i][0] as i64);
        }

        f[0]
    }
}
