pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut ans = 0;
        let mut pos0 = vec![-1];
        let mut total1 = 0;
        for (r, c) in s.chars().enumerate() {
            if c == '0' {
                pos0.push(r as i32);
            } else {
                total1 += 1;
                ans += r as i32 - pos0.last().unwrap();
            }

            let m = pos0.len() as i32;
            for i in (1..m).rev() {
                if (m - i) * (m - i) <= total1 {
                    let p = pos0[i as usize - 1];
                    let q = pos0[i as usize];
                    let cnt0 = m - i;
                    let cnt1 = r as i32 - q + 1 - cnt0;
                    ans += 0.max(q - 0.max(cnt0 * cnt0 - cnt1) - p);
                } else {
                    break;
                }
            }
        }

        ans
    }
}
