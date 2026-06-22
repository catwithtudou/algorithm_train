pub struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut cnt = vec![0; 26];

        for c in text.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }

        cnt[(b'a' - b'a') as usize]
            .min(cnt[(b'b' - b'a') as usize])
            .min(cnt[(b'l' - b'a') as usize] / 2)
            .min(cnt[(b'o' - b'a') as usize] / 2)
            .min(cnt[(b'n' - b'a') as usize])
    }
}