pub struct Solution;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut cnt = [0; 26];
        let (mut max_vowel_cnt, mut max_consonant_cnt) = (0, 0);

        for ch in s.bytes() {
            let idx = (ch as u8 - b'a') as usize;
            cnt[idx] += 1;
            if "aeiou".contains(ch as char) {
                max_vowel_cnt = max_vowel_cnt.max(cnt[idx]);
            } else {
                max_consonant_cnt = max_consonant_cnt.max(cnt[idx]);
            }
        }
        max_vowel_cnt + max_consonant_cnt
    }
}
