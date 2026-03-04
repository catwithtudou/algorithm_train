pub struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for row in &mat {
            let row_sum: i32 = row.iter().sum();
            if row_sum != 1 {
                continue;
            }

            if let Some(j) = row.iter().position(|&x| x == 1) {
                let mut col_sum = 0;
                for r in &mat {
                    col_sum += r[j];
                }
                if col_sum == 1 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
