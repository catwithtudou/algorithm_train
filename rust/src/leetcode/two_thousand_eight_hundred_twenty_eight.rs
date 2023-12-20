pub struct Solution;

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        words.len() == s.len() && words.into_iter().zip(s.as_bytes()).all(|(word, &c)| word.as_bytes()[0] == c)
    }
}