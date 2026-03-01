pub struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        (n.as_bytes().iter().max().unwrap() - b'0') as i32
    }
}