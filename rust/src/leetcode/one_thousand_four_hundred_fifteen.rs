pub struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        if k > 3 << (n - 1) {
            return String::new();
        }

        k -= 1;
        let n = n as usize;
        let mut ans = vec![0; n];
        ans[0] = b'a' + (k >> (n - 1)) as u8;
        for i in 1..n {
            ans[i] = b'a' + (k >> (n - 1 - i) & 1) as u8;
            if ans[i] >= ans[i - 1] {
                ans[i] += 1;
            }
        }

        unsafe { String::from_utf8_unchecked(ans) }
    }
}
