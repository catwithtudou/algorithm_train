pub struct Solution;

impl Solution {
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = box_grid.len();
        let n = box_grid[0].len();
        let mut ans = vec![vec!['\0'; m]; n];

        for (i, row) in box_grid.iter().enumerate() {
            let mut cnt = 0;
            for (j, &ch) in row.into_iter().enumerate() {
                let mut ch = ch;
                if ch == '#' {
                    cnt += 1;
                    ch = '.';
                }

                ans[j][m - 1 - i] = ch;

                if j == n - 1 || row[j + 1] == '*' {
                    for k in j - cnt + 1..=j {
                        ans[k][m - 1 - i] = '#';
                    }
                    cnt = 0;
                }
            }
        }

        ans
    }
}
