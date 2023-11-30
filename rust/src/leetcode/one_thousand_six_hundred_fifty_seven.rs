pub struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut i_mask = 0;
        let mut i_list = [0; 26];
        for c in word1.as_bytes() {
            let c = (c - b'a') as usize;
            i_mask |= 1 << c;
            i_list[c] += 1;
        }

        let mut j_mask = 0;
        let mut j_list = [0; 26];
        for c in word2.as_bytes() {
            let c = (c - b'a') as usize;
            j_mask |= 1 << c;
            j_list[c] += 1;
        }

        i_list.sort_unstable();
        j_list.sort_unstable();

        i_mask == j_mask && i_list == j_list
    }
}