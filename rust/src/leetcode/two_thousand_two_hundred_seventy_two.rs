pub struct Solution;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let mut ans = 0;

        for a in b'a'..=b'z' {
            for b in b'a'..=b'z' {
                if b == a {
                    continue;
                }

                let (mut f0, mut f1) = (0, i32::MIN);

                for ch in s.bytes() {
                    if ch == a {
                        f0=f0.max(0)+1;
                        f1+=1;
                    }else if ch == b {
                        f0=f0.max(0)-1;
                        f1=f0;
                    }
                    ans=ans.max(f1);
                }
            }
        }

        ans
    }
}
