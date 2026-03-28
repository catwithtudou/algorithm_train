pub struct Solution;

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut s = vec![0; n];
        let mut i = 0;
        for c in b'a'..=b'z' {
            for j in i..n {
                if lcp[i][j] > 0 {
                    s[j] = c;
                }
            }
            while i < n && s[i] > 0 {
                i += 1;
            }
            if i == n {
                break;
            }
        }

        if i < n {
            return String::new();
        }

        for i in (0..n).rev() {
            for j in (0..n).rev() {

                let actual_lcp = if s[i]!=s[j] {
                    0
                }else if i==n-1 || j==n-1 {
                    1
                }else {
                    lcp[i+1][j+1] + 1
                };

                if lcp[i][j] != actual_lcp {
                    return String::new();
                }

            }
        }

        unsafe { String::from_utf8_unchecked(s) }
     }
}