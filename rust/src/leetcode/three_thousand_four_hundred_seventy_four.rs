pub struct Solution;

impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let n = str1.len();
        let m = str2.len();

        let s1 = str1.as_bytes();
        let t = str2.as_bytes();

        let mut ans = vec![b'?'; n + m - 1];

        // 先处理所有 'T'，强制把对应区间设置成 str2
        for (i, &ch) in s1.iter().enumerate() {
            if ch != b'T' {
                continue;
            }

            for j in 0..m {
                if ans[i + j] != b'?' && ans[i + j] != t[j] {
                    return String::new();
                }
                ans[i + j] = t[j];
            }
        }

        // 记录初始状态，后面处理 'F' 时要知道哪些位置原本是 '?'
        let old_ans = ans.clone();

        // 剩余 '?' 全部先填成 'a'
        for ch in &mut ans {
            if *ch == b'?' {
                *ch = b'a';
            }
        }

        // 处理所有 'F'
        for (i, &ch) in s1.iter().enumerate() {
            if ch != b'F' {
                continue;
            }

            let sub = &ans[i..i + m];
            if sub != t {
                continue;
            }

            // 如果当前子串恰好等于 str2，就必须改掉一位
            // 优先从右往左找原本是 '?' 的位置，改成 'b'
            let mut changed = false;
            for j in (0..m).rev() {
                if old_ans[i + j] == b'?' {
                    ans[i + j] = b'b';
                    changed = true;
                    break;
                }
            }

            if !changed {
                return String::new();
            }
        }

        String::from_utf8(ans).unwrap()
    }
}