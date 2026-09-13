pub struct Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len() as isize;
        let mut ans = 0;

        for dx in 1 - n..n {
            for dy in 1 - n..n {
                let mut count = 0;

                let row_start = (-dx).max(0);
                let row_end = (n - dx).min(n);

                let col_start = (-dy).max(0);
                let col_end = (n - dy).min(n);

                for i in row_start..row_end {
                    for j in col_start..col_end {
                        count += img1[i as usize][j as usize]
                            * img2[(i + dx) as usize][(j + dy) as usize];
                    }
                }

                ans = ans.max(count);
            }
        }

        ans
    }
}