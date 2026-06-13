pub struct Solution;

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut ans = String::with_capacity(words.len());

        for w in words {
            let mut sum = 0;

            for c in w.as_bytes() {
                let idx = (*c - b'a') as usize;
                sum += weights[idx];
            }

            let ch = (b'z' - (sum % 26) as u8) as char;
            ans.push(ch);
        }

        ans
    }
}
