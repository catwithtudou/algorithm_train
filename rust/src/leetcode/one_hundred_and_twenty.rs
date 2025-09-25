pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        fn dfs(i: usize, j: usize, triangle: &[Vec<i32>], memo: &mut [Vec<i32>]) -> i32 {
            if i == triangle.len() - 1 {
                return triangle[i][j];
            }
            if memo[i][j] != i32::MIN {
                return memo[i][j];
            }
            memo[i][j] = dfs(i + 1, j, triangle, memo).min(dfs(i + 1, j + 1, triangle, memo))
                + triangle[i][j];
            memo[i][j]
        }
        let n = triangle.len();
        let mut memo = vec![vec![i32::MIN; n]; n];
        dfs(0, 0, &triangle, &mut memo)
    }
}
