pub struct Solution;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut ans = 0;
        n /= (n & -n) * 2;
        while n > 0 {
            let gap = n.trailing_zeros() as i32 + 1;
            ans = ans.max(gap);
            n >>= gap;
        }
        ans as _
    }
}
