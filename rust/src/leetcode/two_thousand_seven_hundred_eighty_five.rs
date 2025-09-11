pub struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        const VOWEL_MASK: u32 = 0x208222;
        let mut cnt = [0; 'z' as usize + 1];
        for ch in s.bytes() {
            if (VOWEL_MASK >> (ch & 31)) & 1 > 0 {
                cnt[ch as usize] += 1;
            }
        }

        let mut s = s.into_bytes();
        let mut j = 0;
        for ch in s.iter_mut() {
            if VOWEL_MASK >> (*ch & 31) & 1 == 0 {
                continue;
            }

            while cnt[j as usize] == 0 {
                if j == b'Z' {
                    j = b'a';
                } else {
                    j += 1;
                }
            }

            *ch = j;
            cnt[j as usize] -= 1;
        }
        unsafe {String::from_utf8_unchecked(s)}
    }
}
