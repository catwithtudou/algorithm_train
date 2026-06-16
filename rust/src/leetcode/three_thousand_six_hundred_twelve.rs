pub struct Solution;

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut ans: Vec<u8> = Vec::new();

        for c in s.bytes() {
            match c {
                b'*' => {
                    ans.pop();
                }
                b'#' => {
                    let copy = ans.clone();
                    ans.extend(copy);
                }
                b'%' => {
                    ans.reverse();
                }
                _ => {
                    ans.push(c);
                }
            }
        }

        String::from_utf8(ans).unwrap()
    }
}