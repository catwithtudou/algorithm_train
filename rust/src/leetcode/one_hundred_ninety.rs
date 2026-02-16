pub struct Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut x = n ;
        let mut rev = 0;

        for i in 0..32 {
            if x==0 {
                break;
            }
            rev|=(x&1)<<(31-i);
            x>>=1;
        }
        rev
    }
}