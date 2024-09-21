pub struct Solution;

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut score = vec![0i64; edges.len()];
        for (i, &to) in edges.iter().enumerate() {
            let to = to as usize;
            score[to] += i as i64;
            if (score[to] > score[ans]) || (score[to] == score[ans] && to < ans) {
                ans = to;
            }
        }
        ans as _
    }
}
