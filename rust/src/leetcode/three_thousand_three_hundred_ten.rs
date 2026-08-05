pub struct Solution;

impl Solution {
    pub fn remaining_methods(
        n: i32,
        k: i32,
        invocations: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;

        let mut graph = vec![Vec::<usize>::new(); n];

        for edge in &invocations {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            graph[from].push(to);
        }

        // 从方法 k 出发，所有能到达的方法都是可疑方法
        let mut suspicious = vec![false; n];
        let mut stack = vec![k];
        suspicious[k] = true;

        while let Some(x) = stack.pop() {
            for &y in &graph[x] {
                if !suspicious[y] {
                    suspicious[y] = true;
                    stack.push(y);
                }
            }
        }

        // 如果存在非可疑方法调用可疑方法，
        // 那么这批可疑方法无法整体删除，只能保留全部方法
        for edge in &invocations {
            let from = edge[0] as usize;
            let to = edge[1] as usize;

            if !suspicious[from] && suspicious[to] {
                return (0..n as i32).collect();
            }
        }

        // 返回所有非可疑方法
        suspicious
            .into_iter()
            .enumerate()
            .filter_map(|(i, is_suspicious)| {
                (!is_suspicious).then_some(i as i32)
            })
            .collect()
    }
}