pub struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let rem = mean * (n + rolls.len() as i32) - rolls.iter().sum::<i32>();
        if rem < n || rem > n * 6 {
            return vec![];
        }
        let (avg, extra) = (rem / n, (rem % n) as usize);
        let mut ans = vec![avg + 1; extra];
        ans.extend(vec![avg; (n as usize) - extra]);
        ans
    }
}
