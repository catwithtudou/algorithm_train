pub struct Solution;

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();
        let mut ans = vec![folder[0].clone()];
        for s in folder.into_iter().skip(1) {
            let last = ans.last().unwrap();
            if !s.starts_with(last) || s.as_bytes()[last.len()] != b'/' {
                ans.push(s);
            }
        }
        ans
    }
}
