pub struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        fn combination(n: usize, m: usize, cap: i64) -> i64 {
            let m = m.min(n - m);
            let mut res = 1_i128;

            for i in 1..=m {
                res = res * (n + 1 - i) as i128 / i as i128;

                if res >= cap as i128 {
                    return cap;
                }
            }

            res as i64
        }

        // 计算由 cnt 中字符组成的不同排列数量。
        // 结果达到 cap 后直接截断，避免溢出和无效计算。
        fn permutations(cnt: &[usize; 26], mut size: usize, cap: i64) -> i64 {
            let mut res = 1_i64;

            for &c in cnt {
                if c == 0 {
                    continue;
                }

                let ways = combination(size, c, cap);
                res = ((res as i128 * ways as i128).min(cap as i128)) as i64;

                if res >= cap {
                    return cap;
                }

                size -= c;
            }

            res
        }

        let bytes = s.as_bytes();
        let n = bytes.len();
        let half_len = n / 2;
        let mut k = k as i64;

        let mut cnt = [0usize; 26];

        for &b in &bytes[..half_len] {
            cnt[(b - b'a') as usize] += 1;
        }

        // 不足 k 种排列
        if permutations(&cnt, half_len, k) < k {
            return String::new();
        }

        // 贪心构造字典序第 k 小的左半部分
        let mut left = Vec::with_capacity(half_len);

        for i in 0..half_len {
            for ch in 0..26 {
                if cnt[ch] == 0 {
                    continue;
                }

                // 假设当前位置选择字符 ch
                cnt[ch] -= 1;

                let ways = permutations(&cnt, half_len - i - 1, k);

                if ways >= k {
                    left.push(b'a' + ch as u8);
                    break;
                }

                // 跳过所有以当前字符开头的排列
                k -= ways;
                cnt[ch] += 1;
            }
        }

        let mut ans = Vec::with_capacity(n);
        ans.extend_from_slice(&left);

        if n % 2 == 1 {
            ans.push(bytes[half_len]);
        }

        ans.extend(left.iter().rev().copied());

        String::from_utf8(ans).unwrap()
    }
}