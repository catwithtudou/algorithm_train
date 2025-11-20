pub struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut st = vec![(-2, -2, 0)];

        for p in intervals {
            let (start, end) = (p[0], p[1]);

            let idx = st.partition_point(|t| t.0 < start);

            let i = idx - 1;

            let current_total = st.last().unwrap().2;
            let prev_total = st[i].2;
            let mut d = 2 - (current_total - prev_total);

            if start <= st[i].1 {
                d -= st[i].1 - start + 1;
            }

            if d <= 0 {
                continue;
            }

            while end - st.last().unwrap().1 <= d {
                let top = st.pop().unwrap();
                d += top.1 - top.0 + 1;
            }

            let last_s = st.last().unwrap().2;
            st.push((end - d + 1, end, last_s + d));
        }

        st.last().unwrap().2
    }
}
