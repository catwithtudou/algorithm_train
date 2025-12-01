pub struct Solution;

impl Solution {
    pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
        // 1. 排序 (Sort)
        // 使用 sort_unstable 通常比 sort 更快，且对于基本类型效果一致
        batteries.sort_unstable();

        // 2. 计算总和 (Calculate Sum)
        // 注意：必须先转换为 i64 避免溢出
        let mut sum: i64 = batteries.iter().map(|&x| x as i64).sum();

        // 将 n 转换为 i64 以便进行后续计算
        let mut n = n as i64;

        // 3. 反向遍历 (Loop backwards)
        // 对应 Go 代码中的 for i := len(batteries) - 1; ; i--
        for &b in batteries.iter().rev() {
            let val = b as i64;

            // 如果当前最大的电池电量 <= 平均剩余电量
            // 说明剩下的电池足够平均分配给剩下的电脑
            if val <= sum / n {
                return sum / n;
            }

            // 否则，这块电池电量太多，可以单独供一台电脑一直运行
            // 将其从总电量中减去，并减少一台需要供电的电脑
            sum -= val;
            n -= 1;
        }

        0 // 理论上逻辑会在循环内返回，这里是为了通过编译器的检查
    }
}