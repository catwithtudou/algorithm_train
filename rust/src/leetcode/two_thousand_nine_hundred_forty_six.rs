pub struct Solution;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let n = mat[0].len();
        let k = k  as usize;
        for row in mat {
            for j in 0..n {
                if row[j] != row[(j+k) % n ] {
                    return false;
                }
            }
        }
        return true;
    }
}