pub struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut pre_cnt = 0;

        for row in bank {
            let cnt = row.bytes().filter(|&c| c == b'1').count() as i32;
            if cnt > 0 {
                ans += pre_cnt * cnt;
                pre_cnt = cnt;
            }
        }

        ans
    }
}
