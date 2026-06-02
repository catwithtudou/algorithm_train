pub struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        fn solve(
            first_start_time: &[i32],
            first_duration: &[i32],
            second_start_time: &[i32],
            second_duration: &[i32],
        ) -> i32 {
            let mut min_finish = i32::MAX;

            for i in 0..first_start_time.len() {
                min_finish = min_finish.min(first_start_time[i] + first_duration[i]);
            }

            let mut res = i32::MAX;

            for i in 0..second_start_time.len() {
                res = res.min(second_start_time[i].max(min_finish) + second_duration[i]);
            }

            res
        }

        let land_water = solve(
            &land_start_time,
            &land_duration,
            &water_start_time,
            &water_duration,
        );

        let water_land = solve(
            &water_start_time,
            &water_duration,
            &land_start_time,
            &land_duration,
        );

        land_water.min(water_land)
    }
}