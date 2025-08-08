pub struct Solution;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        // 当n足够大时，概率接近1.0
        if n >= 4451 {
            return 1.0;
        }

        // 将n按25为单位进行缩放
        let n = (n + 24) / 25;
        let n = n as usize;

        // 创建记忆化数组
        let mut memo = vec![vec![0.0; n + 1]; n + 1];

        // 递归函数定义
        fn dfs(a: usize, b: usize, memo: &mut Vec<Vec<f64>>) -> f64 {
            // 如果两个汤都用完了，返回0.5
            if a == 0 && b == 0 {
                return 0.5;
            }
            // 如果A汤用完了（B汤没用完），返回1.0
            if a == 0 {
                return 1.0;
            }
            // 如果B汤用完了（A汤没用完），返回0.0
            if b == 0 {
                return 0.0;
            }

            // 如果已经计算过，直接返回缓存结果
            if memo[a][b] != 0.0 {
                return memo[a][b];
            }

            // 计算四种操作的期望值
            let mut result = 0.0;

            // 操作1: A减4, B不变
            result += dfs(a.saturating_sub(4), b, memo);
            // 操作2: A减3, B减1
            result += dfs(a.saturating_sub(3), b.saturating_sub(1), memo);
            // 操作3: A减2, B减2
            result += dfs(a.saturating_sub(2), b.saturating_sub(2), memo);
            // 操作4: A减1, B减3
            result += dfs(a.saturating_sub(1), b.saturating_sub(3), memo);

            // 每种操作的概率是1/4
            result /= 4.0;

            // 缓存结果
            memo[a][b] = result;
            result
        }

        dfs(n, n, &mut memo)
    }
}