pub struct Solution;

impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|e| e[1]);

        let mut st: Vec<(i32, i32)> = vec![(0, 0)];
        let mut ans = 0;

        for e in events {
            let (start, end, val) = (e[0], e[1], e[2]);

            let idx = st.partition_point(|x| x.0 < start);

            if idx > 0 {
                ans = ans.max(st[idx - 1].1 + val);
            }

            if val > st.last().unwrap().1 {
                st.push((end, val));
            }
        }

        ans
    }
}
