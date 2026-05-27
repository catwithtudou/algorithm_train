pub struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut state = vec![0; 27];
        let mut ans = 0;

        for &c in word.as_bytes() {
            let x = (c & 31) as usize;

            if (c & 32) > 0 {
                if state[x] == 0 {
                    state[x] = 1;
                } else if state[x] == 2 {
                    state[x] = -1;
                    ans -= 1;
                }
            } else {
                if state[x] == 0 {
                    state[x] = -1;
                } else if state[x] == 1 {
                    state[x] = 2;
                    ans += 1;
                }
            }
        }

        ans
    }
}
