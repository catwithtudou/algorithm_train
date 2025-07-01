pub struct Solution;

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        1 + word
            .into_bytes()
            .windows(2)
            .filter(|w| w[0] == w[1])
            .count() as i32
    }
}
