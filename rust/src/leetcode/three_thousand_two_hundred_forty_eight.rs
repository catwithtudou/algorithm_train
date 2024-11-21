pub struct Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let (mut i, mut j) = (0, 0);
        for s in commands {
            match s.as_bytes()[0] {
                b'U' => i -= 1,
                b'D' => i += 1,
                b'L' => j -= 1,
                b'R' => j += 1,
                _ => {}
            }
        }
        i * n + j
    }
}
