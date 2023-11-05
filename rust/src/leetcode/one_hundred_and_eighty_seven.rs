use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }
        let mut cnt = HashMap::new();
        let mut ans = Vec::new();
        for i in 0..s.len() - 9 {
            let t = &s[i..i + 10];
            let count = cnt.entry(t).or_insert(0);
            *count += 1;
            if *count == 2 {
                ans.push(t.to_string());
            }
        }

        ans
    }
}


#[cfg(test)]
mod one_hundred_and_eighty_seven_test {
    use super::*;

    #[test]
    fn one_hundred_and_eighty_seven() {
        assert_eq!(Solution::find_repeated_dna_sequences(String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT")), vec![String::from("AAAAACCCCC"), String::from("CCCCCAAAAA")]);
    }
}