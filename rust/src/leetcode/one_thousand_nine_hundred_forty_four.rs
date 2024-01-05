pub struct Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut sk = Vec::new();
        let mut res = vec![0; n];
        for (i, &h) in heights.iter().enumerate().rev() {
            while !sk.is_empty() && *sk.last().unwrap() <= h {
                sk.pop();
                res[i] += 1;
            }
            if !sk.is_empty() {
                res[i] += 1;
            }
            sk.push(h);
        }

        res
    }
}