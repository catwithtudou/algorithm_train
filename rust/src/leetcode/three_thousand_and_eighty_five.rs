pub struct Solution;

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut cnt = vec![0; 26];
        for c in word.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        cnt.sort_unstable();

        let mut max_save = 0;

        for (i, &base) in cnt.iter().enumerate() {
            let mut sum = 0;
            for &c in &cnt[i..] {
                sum += c.min(base + k);
            }
            max_save = max_save.max(sum);
        }
        word.len() as i32- max_save
    }
}
