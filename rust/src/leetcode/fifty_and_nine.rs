pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; n as usize]; n as usize];
        let (mut start_x, mut start_y) = (0, 0);
        let mid = (n / 2) as usize;
        let (mut offset, mut count) = (1, 1);
        for _ in (0..(n / 2)).rev() {
            let (mut i, mut j) = (start_x, start_y);

            for _ in start_y..(n as usize - offset) {
                result[start_x][j] = count;
                count += 1;
                j += 1;
            }

            for _ in start_x..(n as usize - offset) {
                result[i][j] = count;
                count += 1;
                i += 1;
            }

            for _ in 0..(j - start_y) {
                result[i][j] = count;
                count += 1;
                j -= 1;
            }

            for _ in 0..(i - start_x) {
                result[i][j] = count;
                count += 1;
                i -= 1;
            }

            start_x += 1;
            start_y += 1;
            offset += 1;
        }

        if n % 2 > 0 {
            result[mid][mid] = count;
        }
        result
    }
}