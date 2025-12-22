pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let m = strs[0].len();
        let mut f = vec![0; m];

        for i in 0..m {
            for j in 0..i {
                if f[j] > f[i] && strs.iter().all(|s| s.as_bytes()[j] <= s.as_bytes()[i]) {
                    f[i] = f[j];
                }
            }
            f[i] += 1;
        }

        m as i32 - f.into_iter().max().unwrap()
    }
}
