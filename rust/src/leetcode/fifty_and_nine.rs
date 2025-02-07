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

    pub fn generate_matrix_other(n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; n as usize]; n as usize];
        let (mut l, mut r, mut b, mut t, mut val) = (0, n - 1, n - 1, 0, 1);
        while val <= n * n {
            for i in l..=r {
                ans[t as usize][i as usize] = val;
                val += 1;
            }
            t += 1;
            for i in t..=b {
                ans[i as usize][r as usize] = val;
                val += 1;
            }
            r -= 1;
            for i in (l..=r).rev() {
                ans[b as usize][i as usize] = val;
                val += 1;
            }
            b -= 1;
            for i in (t..=b).rev() {
                ans[i as usize][l as usize] = val;
                val += 1;
            }
            l += 1;
        }

        ans
    }
}
