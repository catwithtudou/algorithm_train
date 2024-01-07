pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let a = 'a';
        let mut record = vec![0; 26];

        for byte in magazine.bytes() {
            record[byte as usize - a as usize] += 1;
        }

        for byte in ransom_note.bytes() {
            if record[byte as usize - a as usize] == 0 {
                return false;
            }
            record[byte as usize - a as usize] -= 1;
        }

        true
    }
}