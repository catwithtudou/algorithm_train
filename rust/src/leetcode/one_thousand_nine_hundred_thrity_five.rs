pub struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut mask = 0u32;
        for c in broken_letters.chars() {
            mask |= 1 << (c as u8 - b'a');
        }

        let (mut ans, mut ok) = (0, 1);

        for c in text.chars() {
            if c == ' ' {
                ans += ok;
                ok = 1;
            } else if (mask >> (c as u8 - b'a')) & 1 > 0 {
                ok = 0;
            }
        }

        ans += ok;
        ans
    }
}
