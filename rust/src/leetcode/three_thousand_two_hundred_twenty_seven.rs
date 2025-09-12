pub struct Solution;

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.contains(|c: char| "aeiou".contains(c))
    }
}
