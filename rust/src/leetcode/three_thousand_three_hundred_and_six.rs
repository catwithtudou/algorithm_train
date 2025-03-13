pub struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        const VOWEL_MASK: u32 = 1065233;
        let s = word.as_bytes();
        let mut cnt_vowel1 = [0; (b'u' - b'a' + 1) as usize];
        let mut cnt_vowel2 = [0; (b'u' - b'a' + 1) as usize];
        let (mut size_vowel1,mut size_vowel2) = (0,0);
        let (mut cnt_constant1,mut cnt_constant2) = (0,0);
        let (mut left1,mut left2) = (0,0);
        let mut ans: i64 = 0;

        for &ch in s {
            let b = (ch - b'a') as usize;

            if (VOWEL_MASK >> b) & 1 > 0 {
                if cnt_vowel1[b] == 0 {
                    size_vowel1 += 1;
                }
                cnt_vowel1[b] += 1;
                if cnt_vowel2[b] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel2[b] += 1;
            } else {
                cnt_constant1 += 1;
                cnt_constant2 += 1;
            }

            while size_vowel1 == 5 && cnt_constant1 >= k {
                let out = (s[left1] - b'a') as usize;
                if (VOWEL_MASK >> out) & 1 > 0 {
                    cnt_vowel1[out] -= 1;
                    if cnt_vowel1[out] == 0 {
                        size_vowel1 -= 1;
                    }
                } else {
                    cnt_constant1 -= 1;
                }
                left1 += 1;
            }


            while size_vowel2 == 5 && cnt_constant2 > k {
                let out = (s[left2] - b'a') as usize;
                if (VOWEL_MASK >> out) & 1 > 0 {
                    cnt_vowel2[out] -= 1;
                    if cnt_vowel2[out] == 0 {
                        size_vowel2 -= 1;
                    }
                } else {
                    cnt_constant2 -= 1;
                }
                left2 += 1;
            }

            ans += (left1 - left2) as i64;
        }

        ans
    }
}
