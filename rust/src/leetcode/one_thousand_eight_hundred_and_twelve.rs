pub struct Solution;


impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        coordinates.chars().nth(0).unwrap() as u8 & 1 != coordinates.chars().nth(1).unwrap() as u8 & 1
    }
}