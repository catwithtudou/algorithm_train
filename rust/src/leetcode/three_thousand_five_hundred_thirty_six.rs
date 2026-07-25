pub struct Solution;

impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut digits: Vec<i32> = n
            .to_string()
            .bytes()
            .map(|b| (b - b'0') as i32)
            .collect();

        digits.sort_unstable();

        let len = digits.len();
        digits[len - 1] * digits[len - 2]
    }
}