pub struct Solution;


impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        (s.bytes().filter(|&c| c==letter as u8).count() * 100 / s.len()) as _
    }
}