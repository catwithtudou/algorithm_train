pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut ans = n;
        let mut i = 0;
        while i < n {
            let start = if i > 0 && ratings[i] > ratings[i - 1] {
                i - 1
            } else {
                i
            };

            while i + 1 < n && ratings[i + 1] > ratings[i] {
                i += 1;
            }
            let top = i;
            while i + 1 < n && ratings[i + 1] < ratings[i] {
                i += 1;
            }
            let inc = top - start;
            let dec = i - top;
            ans += (inc * (inc - 1) + dec * (dec - 1)) / 2 + inc.max(dec);
            i += 1;
        }

        ans as _
    }
}
