pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        // 空闲服务器：按 (权重, 下标) 取最小
        let mut idle: BinaryHeap<Reverse<(i32, usize)>> = servers
            .iter()
            .enumerate()
            .map(|(i, &w)| Reverse((w, i)))
            .collect();
        // 忙碌服务器：按 (空闲时刻, 下标) 取最小
        let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut now: i64 = 0;
        let mut ans = Vec::with_capacity(tasks.len());

        // 把到 now 为止已经完成的服务器放回空闲堆
        let release = |now: i64,
                       idle: &mut BinaryHeap<Reverse<(i32, usize)>>,
                       busy: &mut BinaryHeap<Reverse<(i64, usize)>>| {
            while let Some(&Reverse((t, i))) = busy.peek() {
                if t > now {
                    break;
                }
                busy.pop();
                idle.push(Reverse((servers[i], i)));
            }
        };

        for (i, &t) in tasks.iter().enumerate() {
            now = now.max(i as i64);
            release(now, &mut idle, &mut busy);

            // 没有空闲的就快进到最早有服务器完成的时刻
            if idle.is_empty() {
                now = busy.peek().unwrap().0 .0;
                release(now, &mut idle, &mut busy);
            }

            let Reverse((_, j)) = idle.pop().unwrap();
            busy.push(Reverse((now + t as i64, j)));
            ans.push(j as i32);
        }
        ans
    }
}