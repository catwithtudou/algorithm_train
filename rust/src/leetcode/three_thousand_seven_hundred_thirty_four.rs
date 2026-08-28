pub struct Solution;

impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let s = s.as_bytes();
        let target = target.as_bytes();
        let n = s.len();

        let mut left = [0_i32; 26];

        for &b in s {
            left[(b - b'a') as usize] += 1;
        }

        // 判断当前字符库存是否合法
        fn valid(left: &[i32; 26]) -> bool {
            left.iter().all(|&c| c >= 0)
        }

        // 回文串如果长度为奇数，只能有一个字符出现奇数次
        let mut mid_ch: Option<u8> = None;

        for i in 0..26 {
            if left[i] % 2 == 0 {
                continue;
            }

            if mid_ch.is_some() {
                return String::new();
            }

            mid_ch = Some(b'a' + i as u8);
            left[i] -= 1;
        }

        // 先假设左半部分完全和 target 的左半部分相同
        for &b in &target[..n / 2] {
            left[(b - b'a') as usize] -= 2;
        }

        if valid(&left) {
            let left_half = &target[..n / 2];

            let mut candidate = Vec::with_capacity(n);
            candidate.extend_from_slice(left_half);

            if let Some(ch) = mid_ch {
                candidate.push(ch);
            }

            candidate.extend(left_half.iter().rev().copied());

            // 左半部分一样时，只需要判断构造出的右半部分
            // 是否让整个回文串严格大于 target
            if candidate.as_slice() > target {
                return String::from_utf8(candidate).unwrap();
            }
        }

        // 从右往左找第一个可以“变大”的位置
        for i in (0..n / 2).rev() {
            let current = (target[i] - b'a') as usize;

            // target[i] 不再固定，把这一对字符归还
            left[current] += 2;

            if !valid(&left) {
                continue;
            }

            // 尝试当前位置换成更大的字符
            for next in current + 1..26 {
                if left[next] == 0 {
                    continue;
                }

                left[next] -= 2;

                // 构造左半部分：
                // 前缀保持 target 不变，当前位置稍微变大，
                // 后面用剩余字符按字典序从小到大填充
                let mut half = target[..=i].to_vec();
                half[i] = b'a' + next as u8;

                for (ch, &count) in left.iter().enumerate() {
                    half.extend(
                        std::iter::repeat(b'a' + ch as u8)
                            .take((count / 2) as usize),
                    );
                }

                let mut ans = Vec::with_capacity(n);

                ans.extend_from_slice(&half);

                if let Some(ch) = mid_ch {
                    ans.push(ch);
                }

                ans.extend(half.iter().rev().copied());

                return String::from_utf8(ans).unwrap();
            }
        }

        String::new()
    }
}