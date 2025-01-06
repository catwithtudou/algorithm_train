pub struct Solution;

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
        special.sort_unstable();
        let n = special.len();
        let mut ans = (special[0] - bottom).max(top - special[n - 1]);
        for i in 1..n {
            ans = ans.max(special[i] - special[i - 1] - 1);
        }
        return ans;
    }
}
