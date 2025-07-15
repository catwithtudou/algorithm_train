pub struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }

        let mut f0 = false;
        let mut f1 = false;

        for c in word.chars() {
            if c.is_alphabetic() {
                if "aeiou".contains(c.to_lowercase().next().unwrap()) {
                    f1 = true;
                } else {
                    f0 = true;
                }
            } else if !c.is_numeric() {
                return false;
            }
        }

        f0 && f1
    }
}