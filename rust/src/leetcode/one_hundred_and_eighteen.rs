pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1]];
        for i in 1..num_rows {
            let mut row = vec![1; i as usize + 1];
            for j in 1..i {
                row[j as usize] = res[i as usize - 1][j as usize - 1] + res[i as usize - 1][j as usize];
            }
            res.push(row);
        }
        res
    }
}