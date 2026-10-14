pub struct Solution;

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        if num * 9 < sum {
            return String::new();
        }

        let mut ans = "9".repeat((sum / 9) as usize);

        if sum % 9 != 0 {
            ans.push(char::from_digit((sum % 9) as u32, 10).unwrap());
        }

        ans.push_str(&"0".repeat(num as usize - ans.len()));

        ans
    }
}