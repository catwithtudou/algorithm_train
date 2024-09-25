pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut groups = vec![HashSet::new(); 26];
        for s in ideas {
            groups[(s.as_bytes()[0] - b'a') as usize].insert(s[1..].to_string());
        }

        let mut ans = 0i64;

        for a in 1..26 {
            for b in 0..a {
                let m = groups[a].iter().filter(|&s| groups[b].contains(s)).count();
                ans += (groups[a].len() - m) as i64 * (groups[b].len() - m) as i64;
            }
        }

        ans * 2
    }
}
