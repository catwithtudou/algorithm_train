pub struct Solution;

impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        if x.min(y / 4) % 2 != 0 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}
