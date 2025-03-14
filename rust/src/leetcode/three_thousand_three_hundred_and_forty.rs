pub struct Solution;

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut s = 0;
        for (i, c) in num.bytes().enumerate() {
            let c = (c - b'0') as i32;
            if i % 2 == 0 {
                s += c;
            } else {
                s -= c;
            }
        }
        s == 0
    }
}
