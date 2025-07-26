pub struct Solution;

impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut group = vec![vec![]; n + 1];

        for pair in &conflicting_pairs {
            let mut a = pair[0] as usize;
            let mut b = pair[1] as usize;
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            group[a].push(b);
        }

        let mut extra = vec![0; n + 2];
        let mut b = vec![n + 1, n + 1];
        let mut ans = 0;

        for i in (1..=n).rev() {
            b.extend_from_slice(&group[i]);
            b.sort();
            b.truncate(2);

            ans += b[0] - i;
            extra[b[0]] += b[1] - b[0];
        }

        let max_extra = extra.iter().max().unwrap_or(&0);

        (ans + max_extra) as i64
    }
}
