pub struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut total = 0;
        let mut neg_cnt = 0;
        let mut mn = i32::MAX;
        for row in matrix {
            for mut x in row {
                if x < 0 {
                    neg_cnt += 1;
                    x = -x; // 先把负数都变成正数
                }
                mn = mn.min(x);
                total += x as i64;
            }
        }

        if neg_cnt % 2 > 0 {
            total -= (mn as i64) * 2;
        }

        total
    }
}
