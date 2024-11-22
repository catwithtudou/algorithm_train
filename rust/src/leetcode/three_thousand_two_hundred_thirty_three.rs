pub struct Solution;

// Compare similar snippet from rust/src/leetcode/three_thousand_two_hundred_thirty_four.rs:
impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        const MX: usize = 31622; // 使用常量
        let mut pi = [0; MX + 1];
        for i in 2..=MX {
            if pi[i] == 0 {
                pi[i] = pi[i - 1] + 1;
                let mut j = i * i;
                while j <= MX {
                    pi[j] = -1;
                    j += i;
                }
            } else {
                pi[i] = pi[i - 1];
            }
        }

        let r_cnt = pi[f64::sqrt(r as f64) as usize];
        let l_cnt = pi[f64::sqrt((l - 1) as f64) as usize];
        r - l + 1 - (r_cnt - l_cnt)
    }
}
