pub struct Solution;

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 121];
        for age in ages {
            cnt[age as usize] += 1;
        }

        let mut ans = 0;
        let (mut cnt_windows, mut age_y) = (0, 0);
        for age_x in 0..cnt.len() {
            let c = cnt[age_x as usize];
            cnt_windows += c;
            if age_y * 2 <= age_x + 14 {
                cnt_windows -= cnt[age_y as usize];
                age_y += 1;
            }
            if cnt_windows > 0 {
                ans += c * cnt_windows - c;
            }
        }

        ans
    }
}
