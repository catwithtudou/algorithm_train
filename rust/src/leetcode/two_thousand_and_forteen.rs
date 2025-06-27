pub struct Solution;

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        // 检查字符串t在s中是否可以重复k次作为子序列
        let check = |t: &str, k: i32| -> bool {
            let mut i = 0;
            let mut count = k;
            let t_bytes = t.as_bytes();

            for c in s.bytes() {
                if c == t_bytes[i] {
                    i += 1;
                    if i == t_bytes.len() {
                        count -= 1;
                        if count == 0 {
                            return true;
                        }
                        i = 0;
                    }
                }
            }
            false
        };

        // 统计每个字符的出现次数
        let mut cnt = [0; 26];
        for c in s.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }

        // 收集出现次数大于等于k的字符
        let mut cs = Vec::new();
        for c in b'a'..=b'z' {
            if cnt[(c - b'a') as usize] >= k {
                cs.push(c);
            }
        }

        // BFS搜索最长的有效子序列
        let mut q = vec![String::new()];
        let mut ans = String::new();

        while !q.is_empty() {
            let cur = q.remove(0);
            for &c in &cs {
                let nxt = format!("{}{}", cur, c as char);
                if check(&nxt, k) {
                    ans = nxt.clone();
                    q.push(nxt);
                }
            }
        }

        ans
    }
}
