pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        fn dfs(s: &[u8], i: &mut usize) -> BTreeSet<String> {
            let mut res = BTreeSet::new();
            let mut cur = BTreeSet::from([String::new()]);

            while *i < s.len() {
                let ch = s[*i];
                *i += 1;

                match ch {
                    b'}' => break,

                    b',' => {
                        res.append(&mut cur);
                        cur.insert(String::new());
                    }

                    b'{' => {
                        let sub = dfs(s, i);
                        let mut next = BTreeSet::new();

                        for a in &cur {
                            for b in &sub {
                                next.insert(format!("{a}{b}"));
                            }
                        }

                        cur = next;
                    }

                    _ => {
                        let ch = ch as char;

                        cur = cur
                            .into_iter()
                            .map(|mut x| {
                                x.push(ch);
                                x
                            })
                            .collect();
                    }
                }
            }

            res.append(&mut cur);
            res
        }

        let mut i = 0;
        dfs(expression.as_bytes(), &mut i)
            .into_iter()
            .collect()
    }
}