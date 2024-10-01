pub struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let last_day = *days.last().unwrap() as usize;
        let mut is_travel = vec![false; last_day + 1];
        for &day in &days {
            is_travel[day as usize] = true;
        }
        let mut f = vec![0; last_day + 1];
        for i in 1..=last_day {
            if !is_travel[i] {
                f[i] = f[i - 1];
            } else {
                f[i] = (f[i - 1] + costs[0])
                    .min(f[i.max(7) - 7] + costs[1])
                    .min(f[i.max(30) - 30] + costs[2]);
            }
        }
        f[last_day]
    }
}
