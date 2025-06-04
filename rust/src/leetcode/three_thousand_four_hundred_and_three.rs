pub struct Solution;

impl Solution {
    pub fn answer_string(word: String, k: i32) -> String {
        if k == 1 {
            return word;
        }

        let n = word.len();
        let s = word.as_bytes();
        let (mut i, mut j) = (0, 1);

        while j < n {
            let mut k = 0;
            while j + k < n && s[i + k] == s[j + k] {
                k += 1;
            }
            if j + k < n && s[i + k] < s[j + k] {
                let t = i;
                i = j;
                j = (j+1).max(t + k + 1);
            } else {
                j += k + 1;
            }
        }

        let end = n.min(i + n - k as usize + 1);
        word[i..end].to_string()
    }
}
