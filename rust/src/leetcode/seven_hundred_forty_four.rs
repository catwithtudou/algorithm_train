pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let n = letters.len();
        let (mut left, mut right) = (-1, n as i32);
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if letters[mid as usize] > target {
                right = mid;
            } else {
                left = mid;
            }
        }
        if right == n as i32 {
            letters[0]
        } else {
            letters[right as usize]
        }
    }
}
