pub struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut count = [0usize; 26];

        // 统计前半部分字符
        for &b in &bytes[..n / 2] {
            count[(b - b'a') as usize] += 1;
        }

        // 按字典序构造左半部分
        let mut left = Vec::with_capacity(n / 2);

        for (i, &cnt) in count.iter().enumerate() {
            let ch = b'a' + i as u8;
            left.extend(std::iter::repeat(ch).take(cnt));
        }

        let mut ans = Vec::with_capacity(n);
        ans.extend_from_slice(&left);

        // 奇数长度时保留原字符串的中间字符
        if n % 2 == 1 {
            ans.push(bytes[n / 2]);
        }

        // 右半部分是左半部分的逆序
        ans.extend(left.iter().rev().copied());

        String::from_utf8(ans).unwrap()
    }
}