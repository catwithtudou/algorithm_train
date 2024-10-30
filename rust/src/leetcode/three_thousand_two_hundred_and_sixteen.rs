pub struct Solution;

impl Solution {
    pub fn get_smallest_string(mut s: String) -> String {
        unsafe {
            let t = s.as_bytes_mut();
            for i in 1..t.len() {
                let (x,y) = (t[i-1],t[i]);
                if x>y && x%2 == y %2 {
                    t[i-1]=y;
                    t[i]=x;
                    break;
                }
            }
            s
        }
    }
}