pub struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut ans = 0;
        let mut c = 0;


        for b in s.bytes() {
            if b == b'[' {
                c+=1;
            }else if c > 0 {
                c-=1;
            }else{
                c+=1;
                ans+=1;
            }
        }

        ans
    }
}