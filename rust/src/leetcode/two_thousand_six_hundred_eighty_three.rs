pub struct Solution;

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut xor = 0;
        for v in derived {
            xor ^= v;
        }
        xor == 0
    }
}
