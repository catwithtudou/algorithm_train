pub struct Solution;


impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let mut ans = 0;
        let n = matrix[0].len();
        let rows: Vec<u32> = matrix
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .enumerate()
                    .fold(0, |pre, (j, x)| (pre | (x << j) as u32))
            })
            .collect();

        for mask in 0u32..1 << n {
            if mask.count_ones() != num_select as u32 {
                continue;
            }
            let cnt = rows.iter().fold(0, |cnt, &x| cnt + ((mask & x) == x) as i32);
            ans = ans.max(cnt);
        }
        ans
    }
}