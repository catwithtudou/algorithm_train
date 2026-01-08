pub struct Solution;

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut f = vec![vec![i32::MIN; m + 1]; n + 1];

        for (i, x) in nums1.iter().enumerate() {
            for (j, y) in nums2.iter().enumerate() {
                f[i + 1][j + 1] = (f[i][j].max(0) + x * y).max(f[i+1][j]).max(f[i][j+1]);
            }
        }

        f[n][m]
    }
}
