pub struct Solution;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut res = 0;
        let mut word_index = [[false; 26]; 26];
        for s in words {
            let s = s.as_bytes();
            let x = (s[0] - b'a') as usize;
            let y = (s[1] - b'a') as usize;
            if word_index[y][x] {
                res += 1;
                continue;
            }
            word_index[x][y] = true;
        }


        res
    }
}

