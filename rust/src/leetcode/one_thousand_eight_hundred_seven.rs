pub struct Solution;

use std::collections::HashMap;


impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let mp = knowledge.iter().map(|kv| (kv[0].as_str(), kv[1].as_str())).collect::<HashMap<_,_>>();

        let mut ans = vec![];
        let mut left = usize::MAX;
        for (i, ch) in s.bytes().enumerate() {
            if ch == b'(' {
                left = i;
            } else if ch == b')' {
                if let Some(value) = mp.get(&s[left+1..i]) {
                    ans.extend_from_slice(value.as_bytes());
                }else{
                    ans.push(b'?');
                }
                left = usize::MAX;
            } else if left == usize::MAX {
                ans.push(ch);
            }
        }

        unsafe { String::from_utf8_unchecked(ans) }
    }
}
