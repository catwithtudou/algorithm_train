pub struct Solution;

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .into_iter()
            .enumerate()
            .filter_map(|(i, s)| s.contains(x).then_some(i as i32))
            .collect()
    }
}
