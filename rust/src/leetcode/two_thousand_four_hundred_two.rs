pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        // 1. 按照开始时间排序
        meetings.sort_unstable_by_key(|m| m[0]);

        // idle: 存储空闲会议室的编号 (最小堆)
        let mut idle = BinaryHeap::new();
        for i in 0..n {
            idle.push(Reverse(i));
        }

        // using: 存储正在使用的会议室 (最小堆)
        // 存储元素为: (结束时间, 会议室编号)
        // Rust BinaryHeap 会按 Tuple 的第一个元素排序，若相同则看第二个
        let mut using: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        let mut cnt = vec![0; n];

        for m in meetings {
            let start = m[0] as i64;
            let duration = (m[1] - m[0]) as i64;

            // 2. 将已经结束会议的房间释放回 idle
            while let Some(Reverse((end_time, room_idx))) = using.peek() {
                if *end_time <= start {
                    idle.push(Reverse(*room_idx));
                    using.pop();
                } else {
                    break;
                }
            }

            let room_idx: usize;
            let current_end_time: i64;

            if let Some(Reverse(idx)) = idle.pop() {
                // 3. 有空闲房间，直接使用
                room_idx = idx;
                current_end_time = start + duration;
            } else {
                // 4. 无空闲房间，取最早结束的房间
                let Reverse((end_time, idx)) = using.pop().unwrap();
                room_idx = idx;
                current_end_time = end_time + duration;
            }

            cnt[room_idx] += 1;
            using.push(Reverse((current_end_time, room_idx)));
        }

        // 5. 找到举办会议次数最多的房间中编号最小的一个
        let mut ans = 0;
        for i in 1..n {
            if cnt[i] > cnt[ans] {
                ans = i;
            }
        }

        ans as i32
    }
}
