pub struct Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut spaces_copy = spaces.clone();
        spaces_copy.push(s.len() as i32);

        let mut ans = String::with_capacity(s.len() + spaces.len());

        ans.push_str(&s[0..spaces_copy[0] as usize]);

        for i in 1..spaces_copy.len() {
            ans.push(' ');
            ans.push_str(&s[spaces_copy[i-1] as usize..spaces_copy[i] as usize]);
        }

        ans
    }
}
