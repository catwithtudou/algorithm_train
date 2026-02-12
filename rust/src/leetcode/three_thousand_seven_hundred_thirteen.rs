pub struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let mut ans = 0;
        let n = s.len();
        for i in 0..n {
            let mut cnt = vec![0; 26];
            let mut mx = 0;
            let mut kinds = 0;
            for j in i..n {
                let b = (s.as_bytes()[j] - b'a') as usize;
                if cnt[b] == 0 {
                    kinds += 1;
                }
                cnt[b] += 1;
                mx = mx.max(cnt[b]);
                if mx * kinds == j - i + 1 {
                    ans = ans.max(j - i + 1);
                }
            }
        }
        ans as _
    }
}
