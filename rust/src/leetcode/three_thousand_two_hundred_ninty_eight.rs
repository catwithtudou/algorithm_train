pub struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        if word1.len() < word2.len() {
            return 0;
        }

        let mut diff = vec![0; 26];
        for c in word2.bytes() {
            diff[(c - b'a') as usize] += 1;
        }

        let mut less = diff.iter().filter(|&&d| d > 0).count() as i32;

        let (mut ans, mut left) = (0, 0);
        let word1 = word1.as_bytes();

        for c in word1 {
            let c = (c-b'a') as usize;
            diff[c]-=1;
            if diff[c] == 0 {
                less-=1;
            }
            while less == 0 {
                let out_char = (word1[left]-b'a') as usize;
                if diff[out_char]==0 {
                    less+=1;
                }
                diff[out_char]+=1;
                left+=1;
            }
            ans+=left;
        }

        ans as _
    }
}
