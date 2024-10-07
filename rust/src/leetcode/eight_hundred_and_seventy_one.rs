pub struct Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let n = stations.len();
        let mut dp = vec![0;n+1];
        dp[0]=start_fuel;
        for i in 0..n {
            for j in (0..=i).rev() {
                if dp[j]>=stations[i][0] {
                    dp[j+1]=dp[j+1].max(dp[j]+stations[i][1]);
                }
            }
        }
        for i in 0..=n {
            if dp[i]>=target {
                return i as _;
            }
        }

        -1
    }
}