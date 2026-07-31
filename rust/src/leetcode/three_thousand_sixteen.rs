pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut cnt = [0_i32;26];

        for ch in word.bytes() {
            cnt[(ch-b'a') as usize] +=1;
        }

        cnt.sort_unstable_by(|a,b| b.cmp(a));

        cnt.iter().enumerate().map(|(i,&freq)| freq*(i as i32 /8 +1)).sum()

    }
}