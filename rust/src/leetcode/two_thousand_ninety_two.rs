pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        // 1. 按时间对会议进行排序
        // 对应 Go: slices.SortFunc(meetings, func(a, b []int) int { return a[2] - b[2] })
        meetings.sort_unstable_by_key(|k| k[2]);

        // 2. 初始化知晓秘密的人的状态
        // 对应 Go: haveSecret := map[int]bool{0: true, firstPerson: true}
        // 使用 Vec<bool> 替代 Map 以获得 O(1) 的访问速度
        let mut known = vec![false; n as usize];
        known[0] = true;
        known[first_person as usize] = true;

        let m = meetings.len();
        let mut i = 0;

        // 3. 分组循环
        while i < m {
            let time = meetings[i][2];
            let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
            let mut people_in_time_slot: HashSet<i32> = HashSet::new();

            // 构建当前时间点的邻接图
            // 对应 Go: for ; i < m && meetings[i][2] == time; i++ { ... }
            while i < m && meetings[i][2] == time {
                let x = meetings[i][0];
                let y = meetings[i][1];

                graph.entry(x).or_default().push(y);
                graph.entry(y).or_default().push(x);

                people_in_time_slot.insert(x);
                people_in_time_slot.insert(y);

                i += 1;
            }

            // 4. 在当前时间切片内进行传播 (DFS/BFS)
            // 对应 Go: vis := map[int]bool{}, dfs func...
            let mut visited: HashSet<i32> = HashSet::new();

            for &person in &people_in_time_slot {
                // 如果这个人知道秘密，且在当前批次还没处理过，以此为起点传播
                if known[person as usize] && !visited.contains(&person) {
                    // 开始迭代式 DFS
                    let mut stack = vec![person];
                    visited.insert(person);

                    while let Some(curr) = stack.pop() {
                        // 既然能遍历到这里，说明 curr 知道秘密，标记它
                        known[curr as usize] = true;

                        if let Some(neighbors) = graph.get(&curr) {
                            for &next_p in neighbors {
                                if !visited.contains(&next_p) {
                                    visited.insert(next_p);
                                    known[next_p as usize] = true; // 传播秘密
                                    stack.push(next_p);
                                }
                            }
                        }
                    }
                }
            }
        }

        // 5. 收集结果
        // 对应 Go: return slices.Collect(maps.Keys(haveSecret))
        (0..n).filter(|&i| known[i as usize]).collect()
    }
}
