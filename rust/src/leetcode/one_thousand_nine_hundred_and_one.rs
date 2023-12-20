pub struct Solution;

impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut left, mut right) = (0, mat.len() - 1);
        while left < right {
            let i = left + (right - left) / 2;
            let j = Self::row_max(&mat[i]);
            if mat[i][j] > mat[i + 1][j] {
                right = i;
            } else {
                left = i + 1
            }
        }
        vec![left as i32, Self::row_max(&mat[left]) as i32]
    }

    fn row_max(a: &Vec<i32>) -> usize {
        (0..a.len()).max_by_key(|&i| a[i]).unwrap()
    }
}