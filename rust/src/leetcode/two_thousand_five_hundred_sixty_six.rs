pub struct Solution;

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let num_str = num.to_string();
        let mut mx = num;

        for c in num_str.chars() {
            if c != '9' {
                mx = num_str.replace(c, "9").parse::<i32>().unwrap();
                break;
            }
        }

        let first_char = num_str.chars().next().unwrap();
        let mn = num_str.replace(first_char, "0").parse::<i32>().unwrap();

        mx - mn
    }
}
