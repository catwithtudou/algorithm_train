pub struct Solution;

impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        let mut memo =
            vec![vec![vec![(0, 0); (n + 1) as usize]; (n + 1) as usize]; (n + 1) as usize];

        fn dfs(
            n: i32,
            first: i32,
            second: i32,
            memo: &mut Vec<Vec<Vec<(i32, i32)>>>,
        ) -> (i32, i32) {
            let (mut first, mut second) = (first, second);

            if first + second == n + 1 {
                return (1, 1);
            }

            if first + second > n + 1 {
                let temp = first;
                first = n + 1 - second;
                second = n + 1 - temp;
            }

            let cached = memo[n as usize][first as usize][second as usize];
            if cached.0 > 0 {
                return cached;
            }

            let m = (n + 1) / 2;
            let (min_mid, max_mid) = if second > m {
                (second - n / 2 - 1, m - first)
            } else {
                (0, second - first)
            };

            let mut earliest = i32::MAX;
            let mut latest = 0;

            for left in 0..first {
                for mid in min_mid..max_mid {
                    let (r_earliest, r_latest) = dfs(m, left + 1, left + mid + 2, memo);
                    earliest = earliest.min(r_earliest);
                    latest = latest.max(r_latest);
                }
            }

            earliest += 1;
            latest += 1;

            let result = (earliest, latest);
            memo[n as usize][first as usize][second as usize] = result;
            result
        }

        let (earliest, latest) = dfs(n, first_player, second_player, &mut memo);
        vec![earliest, latest]
    }
}
