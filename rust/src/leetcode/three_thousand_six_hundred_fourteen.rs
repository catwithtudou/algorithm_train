pub struct Solution;

impl Solution {
    pub fn process_str(s: String, k: i64) -> char {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut size = vec![0_i64; n];
        let mut sz = 0_i64;

        // 正向计算每一步操作后的字符串长度
        for i in 0..n {
            match bytes[i] {
                b'*' => {
                    sz = (sz - 1).max(0);
                }
                b'#' => {
                    sz *= 2;
                }
                b'%' => {
                    // 反转不改变长度
                }
                _ => {
                    sz += 1;
                }
            }

            size[i] = sz;
        }

        if k >= size[n - 1] {
            return '.';
        }

        let mut k = k;

        // 反向倒推第 k 个字符来自哪里
        for i in (0..n).rev() {
            let c = bytes[i];
            sz = size[i];

            match c {
                b'#' => {
                    // 当前字符串是前一份复制了一遍
                    // 如果 k 落在后一半，就映射回前一半
                    if k >= sz / 2 {
                        k -= sz / 2;
                    }
                }
                b'%' => {
                    // 当前操作是反转，所以位置也要反转回去
                    k = sz - 1 - k;
                }
                b'*' => {
                    // 删除操作，倒推时不用处理
                }
                _ => {
                    // 普通字符只会出现在当前长度的最后一位
                    if k == sz - 1 {
                        return c as char;
                    }
                }
            }
        }

        '.'
    }
}