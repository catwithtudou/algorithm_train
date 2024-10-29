pub struct Solution;

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mask = (1<<n)-1;
        (0..1<<n).filter(|&x| (x>>1)&x==0).map(|x| format!("{:0w$b}",x^mask,w=n as usize)).collect()
    }
}