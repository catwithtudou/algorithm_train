pub struct Solution;

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut cnt1 = [[0; 26]; 2];
        let mut cnt2 = [[0; 26]; 2];

        for (i, v) in s1.chars().enumerate() {
            cnt1[i % 2][(v as u8 - b'a') as usize] += 1;
            cnt2[i % 2][(s2.chars().nth(i).unwrap() as u8 - b'a') as usize] += 1;
        }

        cnt1 == cnt2
    }
}