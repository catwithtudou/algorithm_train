pub struct Solution;


impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut cs: Vec<char> = s.chars().collect();
        let n = cs.len();
        for i in 0..n / 2 {
            let j = n - i - 1;
            if cs[i] == cs[j] {
                continue;
            }
            cs[i] = std::cmp::min(cs[i], cs[j]);
            cs[j] = cs[i];
        }
        cs.into_iter().collect()
    }
}