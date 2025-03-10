pub struct Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let s = num.to_string();
        let k = k as usize;
        let mut ans = 0;

        for i in k..=s.len() {
            let x = s[i - k..i].parse::<i32>().unwrap();
            if x > 0 && num % x == 0 {
                ans += 1;
            }
        }

        ans
    }
}
