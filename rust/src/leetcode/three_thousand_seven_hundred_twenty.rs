pub struct Solution;

impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let s = s.as_bytes();
        let target = target.as_bytes();
        let n = s.len();

        let mut left = [0_i32; 26];

        // left[c] 表示 s 中字符 c 的数量
        // 减去 target 当前前缀已经使用的数量
        for i in 0..n {
            left[(s[i] - b'a') as usize] += 1;
            left[(target[i] - b'a') as usize] -= 1;
        }

        // 从后往前尝试找到第一个可以变大的位置
        for i in (0..n).rev() {
            let current = (target[i] - b'a') as usize;

            // target[i] 不再固定，归还这个字符
            left[current] += 1;

            // 如果前缀已经使用了超过 s 能提供的某个字符，
            // 那么这个位置不能作为修改点
            if left.iter().any(|&count| count < 0) {
                continue;
            }

            // 尝试把 target[i] 换成更大的字符
            for next in current + 1..26 {
                if left[next] == 0 {
                    continue;
                }

                left[next] -= 1;

                // 前 i 个字符保持 target 不变
                let mut ans = target[..=i].to_vec();

                // 第 i 个字符改成更大的字符
                ans[i] = b'a' + next as u8;

                // 剩余字符按字典序最小的方式填充
                for (ch, &count) in left.iter().enumerate() {
                    ans.extend(
                        std::iter::repeat_n(b'a' + ch as u8, count as usize)
                    );
                }

                return String::from_utf8(ans).unwrap();
            }
        }

        String::new()
    }
}