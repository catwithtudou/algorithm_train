pub struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.chars()
            .collect::<Vec<char>>()
            .windows(3)
            .filter(|w| w[0] == w[1] && w[0] == w[2])
            .map(|w| w.iter().collect::<String>())
            .max()
            .unwrap_or("".to_string())
    }
}
