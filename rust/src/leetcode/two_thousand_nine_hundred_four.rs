pub struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let k = k as usize;

        if bytes.iter().filter(|&&b| b == b'1').count() < k {
            return String::new();
        }

        let mut ans = s.clone();
        let mut ones = 0usize;
        let mut left = 0usize;

        for right in 0..bytes.len() {
            if bytes[right] == b'1' {
                ones += 1;
            }

            while ones > k || bytes[left] == b'0' {
                if bytes[left] == b'1' {
                    ones -= 1;
                }
                left += 1;
            }

            if ones == k {
                let candidate = &s[left..=right];

                if candidate.len() < ans.len()
                    || (candidate.len() == ans.len()
                        && candidate < ans.as_str())
                {
                    ans = candidate.to_string();
                }
            }
        }

        ans
    }
}