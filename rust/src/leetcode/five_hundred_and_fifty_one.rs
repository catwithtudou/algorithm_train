pub struct Solution;


impl Solution {
    pub fn check_record(s: String) -> bool {
        s.bytes().filter(|&c|c==b'A').count()<2 && !s.contains("LLL")
    }
}