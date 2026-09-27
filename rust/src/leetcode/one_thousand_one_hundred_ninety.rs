pub struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut links = vec![0usize; n];
        let mut stack = Vec::new();

        // 建立左右括号的双向链接
        for (i, &ch) in bytes.iter().enumerate() {
            match ch {
                b'(' => stack.push(i),
                b')' => {
                    let j = stack.pop().unwrap();
                    links[i] = j;
                    links[j] = i;
                }
                _ => {}
            }
        }

        let mut ans = Vec::with_capacity(n);
        let mut i = 0isize;
        let mut step = 1isize;

        while i >= 0 && i < n as isize {
            let idx = i as usize;

            match bytes[idx] {
                b'(' | b')' => {
                    // 跳到配对括号，并改变遍历方向
                    i = links[idx] as isize;
                    step = -step;
                }
                ch => ans.push(ch),
            }

            i += step;
        }

        String::from_utf8(ans).unwrap()
    }
}