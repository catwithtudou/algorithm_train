pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let n = word.len() as i32;
        let k = n / 8 as i32;
        (k * 4 + n % 8) * (k + 1)
    }
}
