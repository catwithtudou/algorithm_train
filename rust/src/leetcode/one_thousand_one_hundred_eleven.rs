pub struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut ans = Vec::with_capacity(seq.len());
        let mut depth = 0;
        for ch in seq.bytes() {
            if ch == b'(' {
                depth += 1;
                ans.push(depth & 1);
            } else {
                // 先分组再减少深度，保证配对括号属于同一组。
                ans.push(depth & 1);
                depth -= 1;
            }
        }
        ans
    }
}
