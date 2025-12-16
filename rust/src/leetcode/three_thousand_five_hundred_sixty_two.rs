pub struct Solution;

impl Solution {
    pub fn max_profit(
        n: i32,
        present: Vec<i32>,
        future: Vec<i32>,
        hierarchy: Vec<Vec<i32>>,
        budget: i32,
    ) -> i32 {
        let n = n as usize;
        let budget_limit = budget as usize;

        // 1. 构建邻接表 (Adjacency List)
        // hierarchy 中的节点是 1-based，需要转为 0-based
        let mut g = vec![vec![]; n];
        for edge in hierarchy {
            g[edge[0] as usize - 1].push(edge[1] as usize - 1);
        }

        // 2. 定义 DFS 函数
        // 返回值类型: Vec<[i32; 2]>，对应 Go 中的 [][2]int
        // dp[j][0] -> 预算 j，父节点不买
        // dp[j][1] -> 预算 j，父节点买
        fn dfs(
            u: usize,
            g: &Vec<Vec<usize>>,
            present: &Vec<i32>,
            future: &Vec<i32>,
            budget: usize,
        ) -> Vec<[i32; 2]> {
            // 阶段一：汇总子节点 (Group Knapsack)
            // sub_f[j][k] 表示：在预算 j 下，当前节点 u 的状态为 k (0:不买, 1:买) 时，所有子树能提供的最大利润和
            // 注意：这里 u 还没进行购买决策，k 仅仅是用来决定子树能看到的"父节点状态"
            let mut sub_f = vec![[0; 2]; budget + 1];

            for &v in &g[u] {
                let f_v = dfs(v, g, present, future, budget);

                // 分组背包：必须倒序枚举当前总预算 j，防止同一棵子树重复计算
                for j in (0..=budget).rev() {
                    // 枚举分给子树 v 的预算 jy
                    for jy in 0..=j {
                        // 状态转移：更新 u 不买 (0) 和 u 买 (1) 两种情况下的子树累积利润
                        // f_v[jy][0] -> 子树 v 在父节点(即 u)不买的情况下的最大利润
                        // f_v[jy][1] -> 子树 v 在父节点(即 u)买的情况下的最大利润
                        sub_f[j][0] = sub_f[j][0].max(sub_f[j - jy][0] + f_v[jy][0]);
                        sub_f[j][1] = sub_f[j][1].max(sub_f[j - jy][1] + f_v[jy][1]);
                    }
                }
            }

            // 阶段二：决策当前节点 u (Decision)
            // f[j][k] 表示：在预算 j 下，u 的父节点状态为 k 时，以 u 为根的子树的最大利润
            let mut f = vec![[0; 2]; budget + 1];

            for j in 0..=budget {
                for k in 0..2 {
                    // k=0: u 的父节点没买, k=1: u 的父节点买了
                    // 计算当前节点 u 的购买成本
                    // k=0 -> cost = present[u] / 1
                    // k=1 -> cost = present[u] / 2
                    let cost = present[u] / (k + 1);

                    // 选项 1: 不买 u
                    // 收益来自于子树，且子树看到的 u 的状态是不买 (index 0)
                    let not_buy_profit = sub_f[j][0];

                    // 选项 2: 买 u (如果预算足够)
                    if j as i32 >= cost {
                        // 收益 = 子树收益(子树看到 u 买了 -> index 1) + u 的净利润
                        // 预算需要扣除 cost
                        let buy_profit = sub_f[j - cost as usize][1] + future[u] - cost;
                        f[j][k as usize] = not_buy_profit.max(buy_profit);
                    } else {
                        // 买不起只能不买
                        f[j][k as usize] = not_buy_profit;
                    }
                }
            }
            f
        }

        // 3. 执行 DFS 并返回结果
        // 根节点是 0，初始父节点状态自然是"不买"(0)，预算为 budget_limit
        let result = dfs(0, &g, &present, &future, budget_limit);

        result[budget_limit][0]
    }
}
