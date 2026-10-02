pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn dfs(n: i32, left: i32, right: i32, path: &mut String, ans: &mut Vec<String>) {
            if right == n {
                ans.push(path.clone());
                return;
            }
            if left < n {
                path.push('(');
                dfs(n, left + 1, right, path, ans);
                path.pop();
            }
            // 右括号只能匹配已经放入的左括号。
            if right < left {
                path.push(')');
                dfs(n, left, right + 1, path, ans);
                path.pop();
            }
        }

        let mut ans = Vec::new();
        let mut path = String::with_capacity(2 * n as usize);
        dfs(n, 0, 0, &mut path, &mut ans);
        ans
    }
}
