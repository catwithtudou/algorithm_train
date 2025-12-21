pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let (m, n) = (strs.len(), strs[0].len());
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m - 1 {
                if strs[j].as_bytes()[i] > strs[j + 1].as_bytes()[i] {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}
