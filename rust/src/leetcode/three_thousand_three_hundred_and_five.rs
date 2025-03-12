pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        Self::count_f(&word, k) - Self::count_f(&word, k + 1)
    }

    fn count_f(word: &str, k: i32) -> i32 {
        let mut count = 0;
        let mut vowel_counts = HashMap::new();
        let mut non_vowel_count = 0;
        let mut left = 0;
        let bytes = word.as_bytes();

        for right in 0..bytes.len() {
            match bytes[right] {
                b'a' | b'e' | b'i' | b'o' | b'u' => {
                    *vowel_counts.entry(bytes[right]).or_insert(0) += 1;
                }
                _ => {
                    non_vowel_count += 1;
                }
            }

            while vowel_counts.len() == 5 && non_vowel_count >= k {
                match bytes[left] {
                    b'a' | b'e' | b'i' | b'o' | b'u' => {
                        if let Some(val) = vowel_counts.get_mut(&bytes[left]) {
                            *val -= 1;
                            if *val == 0 {
                                vowel_counts.remove(&bytes[left]);
                            }
                        }
                    }
                    _ => {
                        non_vowel_count -= 1;
                    }
                }
                left += 1;
            }

            count += left as i32;
        }

        count
    }
}
