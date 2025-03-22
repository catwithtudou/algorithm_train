pub struct Solution;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut row_idx, mut row_cnt) = (0, 0);

        for (i, row) in mat.iter().enumerate() {
            let s = row.into_iter().sum();
            if s > row_cnt {
                row_idx = i as i32;
                row_cnt = s;
            }
        }

        vec![row_idx, row_cnt]
    }
}
