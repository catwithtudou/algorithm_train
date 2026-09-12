pub struct Solution;

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        #[derive(Clone)]
        struct Interval {
            l: i32,
            r: i32,
            weight: i64,
            idx: i32,
        }

        #[derive(Clone, Default)]
        struct State {
            weight: i64,
            ids: Vec<i32>,
        }

        let mut intervals: Vec<Interval> = intervals
            .into_iter()
            .enumerate()
            .map(|(idx, x)| Interval {
                l: x[0],
                r: x[1],
                weight: x[2] as i64,
                idx: idx as i32,
            })
            .collect();

        intervals.sort_unstable_by_key(|x| x.r);

        let n = intervals.len();
        let mut dp = vec![
            [
                State::default(),
                State::default(),
                State::default(),
                State::default(),
                State::default(),
            ];
            n + 1
        ];

        for i in 0..n {
            let cur = &intervals[i];

            // 找到所有满足 r < cur.l 的区间数量。
            //
            // intervals[..k] 都可以和当前区间共存。
            let k = intervals[..i].partition_point(|x| x.r < cur.l);

            for j in 1..=4 {
                // 不选择当前区间
                let skip = &dp[i][j];

                // 选择当前区间
                let prev = &dp[k][j - 1];
                let take_weight = prev.weight + cur.weight;

                if skip.weight > take_weight {
                    dp[i + 1][j] = skip.clone();
                    continue;
                }

                let mut ids = prev.ids.clone();
                ids.push(cur.idx);
                ids.sort_unstable();

                if skip.weight == take_weight && skip.ids < ids {
                    dp[i + 1][j] = skip.clone();
                } else {
                    dp[i + 1][j] = State {
                        weight: take_weight,
                        ids,
                    };
                }
            }
        }

        dp[n][4].ids.clone()
    }
}