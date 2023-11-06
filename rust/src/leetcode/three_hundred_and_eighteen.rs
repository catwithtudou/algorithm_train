pub struct Solution;


impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let bit_slice: Vec<i32> = words
            .iter()
            .map(|word| {
                word.chars().fold(0, |acc, ch| acc | 1 << (ch as u8 - 'a' as u8))
            }).collect();
        let mut ans = 0;
        for i in 0..bit_slice.len() {
            for j in i + 1..bit_slice.len() {
                if bit_slice[i] & bit_slice[j] == 0 {
                    ans = ans.max(words[i].len() * words[j].len())
                }
            }
        }
        ans as i32
    }
}


#[cfg(test)]
mod three_hundred_and_eighteen_test {
    use super::*;

    #[test]
    fn three_hundred_and_eighteen() {
        assert_eq!(Solution::max_product(vec!["abcw".to_string(), "baz".to_string(), "foo".to_string(), "bar".to_string(), "xtfn".to_string(), "abcdef".to_string()]), 16);
    }
}