pub struct Solution;

impl Solution {
    pub fn reverse_str(mut s: String, k: i32) -> String {
        let  re_s = unsafe { s.as_bytes_mut() };
        let k = k as usize;
        let n = re_s.len();
        for i in (0..n).step_by(2 * k) {
            re_s[i..n.min(i + k)].reverse();
        }
        s
    }
}
