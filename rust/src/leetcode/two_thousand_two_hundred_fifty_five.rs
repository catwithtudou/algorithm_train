pub struct Solution;


impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.into_iter().filter(|w| s.starts_with(w.as_str())).count() as i32
    }
}