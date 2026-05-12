pub struct Solution;

impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by_key(|t| t[0]-t[1]);

        let mut ans = 0;
        let mut s =0;
        for t in tasks {
            let actual = t[0];
            let minimum = t[1];
            ans=ans.max(s+minimum);
            s+=actual;
        }

        ans
    }
}