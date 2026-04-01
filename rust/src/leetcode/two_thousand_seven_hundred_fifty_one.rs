pub struct Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        mut healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut idx = (0..positions.len()).collect::<Vec<_>>();
        idx.sort_unstable_by_key(|&i| positions[i]);

                let directions = directions.as_bytes();
        let mut st = vec![];

        for i in idx {
            if directions[i] == b'R' {
                st.push(i);
                continue;
            }

            while let Some(&j) = st.last() {
                if healths[j] > healths[i] {
                    healths[i] = 0;
                    healths[j]-=1;
                    break;
                } else if healths[i] == healths[j] {
                    healths[i] = 0;
                    healths[j] = 0;
                    st.pop();
                    break;
                }

                healths[i]-=1;
                healths[j] = 0;
                st.pop();
            }
        }

        healths.into_iter().filter(|&h| h > 0).collect()
    }
}
