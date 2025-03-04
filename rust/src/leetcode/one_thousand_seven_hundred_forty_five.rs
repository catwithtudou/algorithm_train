pub struct Solution;

impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        // 检查子串是否为回文
        let is_palindrome = |start: usize, end: usize| -> bool {
            let mut left = start;
            let mut right = end;
            while left < right {
                if chars[left] != chars[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            true
        };

        // memo[i][j] 表示从索引i开始的子串能否分成j段回文
        let mut memo = vec![vec![None; 4]; n]; // k+1 = 4

        Self::dfs(0, 3, &chars, &mut memo, &is_palindrome)
    }
    // 递归函数：从start开始的子串能否分成k段回文
    pub fn dfs(
        start: usize,
        k: usize,
        chars: &[char],
        memo: &mut Vec<Vec<Option<bool>>>,
        is_palindrome: &impl Fn(usize, usize) -> bool,
    ) -> bool {
        if k == 0 {
            return start == chars.len(); // 已分完k段，应该刚好用完整个字符串
        }

        if start >= chars.len() {
            return false; // 字符串已用完，但还需要分段
        }

        // 如果已计算过，直接返回结果
        if let Some(result) = memo[start][k] {
            return result;
        }

        // 尝试所有可能的当前段结束位置
        for end in start..chars.len() {
            if is_palindrome(start, end) && Self::dfs(end + 1, k - 1, chars, memo, is_palindrome) {
                memo[start][k] = Some(true);
                return true;
            }
        }

        memo[start][k] = Some(false);
        false
    }
}
