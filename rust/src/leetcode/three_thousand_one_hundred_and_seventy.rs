pub struct Solution;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut stack = vec![vec![]; 26];
        for (i, &c) in s.as_bytes().iter().enumerate() {
            if c != b'*' {
                stack[(c - b'a') as usize].push(i);
                continue;
            }

            for st in stack.iter_mut() {
                if !st.is_empty() {
                    st.pop();
                    break;
                }
            }
        }

        let mut idx: Vec<usize> = Vec::new();
        for st in &stack {
            idx.extend(st);
        }
        idx.sort();

        let s_bytes = s.as_bytes();
        let ans: Vec<u8> = idx.iter().map(|&i| s_bytes[i]).collect();
        String::from_utf8(ans).unwrap()
    }
}
