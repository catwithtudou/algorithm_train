pub struct Solution;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut cnt = [0; 26];
        for c in s.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        let (mut maxJ, mut minO) = (0, 100);
        for c in cnt {
            if c % 2 > 0 {
                maxJ = maxJ.max(c);
            } else if c > 0 {
                minO = minO.min(c);
            }
        }
        maxJ - minO
    }
}
