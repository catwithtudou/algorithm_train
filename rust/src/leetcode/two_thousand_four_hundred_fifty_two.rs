pub struct Solution;

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut ans = vec![];

        for q in queries {
            for s in &dictionary {
                let mut cnt = 0;
                for (a, b) in q.bytes().zip(s.bytes()) {
                    if a != b {
                        cnt += 1;
                        if cnt > 2 {
                            break;
                        }
                    }
                }
                if cnt <= 2 {
                    ans.push(q);
                    break;
                }
            }
        }

        ans
    }
}
