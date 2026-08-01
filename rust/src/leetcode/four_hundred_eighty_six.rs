pub struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut f = vec![vec![0; n]; n];

        for i in (0..n).rev() {
            f[i][i]=nums[i];
            for j in i+1..n {
                f[i][j]=(nums[i]-f[i+1][j]).max(nums[j]-f[i][j-1])
            }
        }

        f[0][n-1]>=0
    }
}
