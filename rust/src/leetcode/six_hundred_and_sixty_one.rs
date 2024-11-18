pub struct Solution;

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (img.len(), img[0].len());
        let mut sum = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                sum[i + 1][j + 1] = sum[i][j + 1] + sum[i + 1][j] - sum[i][j] + img[i][j];
            }
        }

        let mut ans = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let (a, b, c, d) = (
                    i.saturating_sub(1),
                    j.saturating_sub(1),
                    (i + 1).min(m - 1),
                    (j + 1).min(n - 1),
                );
                let cnt = ((c - a + 1) * (d - b + 1)) as i32;
                let total = sum[c + 1][d + 1] - sum[a][d + 1] - sum[c + 1][b] + sum[a][b];
                ans[i][j] = total / cnt;
            }
        }
        ans
    }
}
