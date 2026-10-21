pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let t = s.as_bytes();

        if t.windows(2).all(|w| w[0] <= w[1]) {
            return 0;
        }

        let n = t.len();
        if n == 2 {
            return -1;
        }

        let mn = *t.iter().min().unwrap();
        let mx = *t.iter().max().unwrap();

        if t[0]==mn || t[n-1] == mx {
            return 1;
        }

        if t[1..n-1].iter().any(|&ch| ch == mn || ch==mx) {
            return 2;
        }

        3
    }
}