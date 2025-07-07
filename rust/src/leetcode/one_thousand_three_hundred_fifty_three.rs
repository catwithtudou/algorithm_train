use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut mx = 0;
        for e in &events {
            mx = mx.max(e[1]);
        }

        let mut cnt = vec![vec![]; (mx + 1) as usize];
        for e in events {
            cnt[e[0] as usize].push(e[1]);
        }

        let mut ans = 0;
        let mut h = BinaryHeap::<i32>::new();

        for (i, g) in cnt.into_iter().enumerate() {
            while let Some(end) = h.peek() {
                if -end >= i as i32 {
                    break;
                }
                h.pop();
            }

            for end in g {
                h.push(-end);
            }

            if let Some(_) = h.pop() {
                ans += 1;
            }
        }

        ans
    }
}
