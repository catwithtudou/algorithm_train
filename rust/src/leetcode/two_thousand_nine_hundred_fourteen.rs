pub struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes().chunks_exact(2).filter(|p| p[0]!=p[1]).count() as _
    }
}