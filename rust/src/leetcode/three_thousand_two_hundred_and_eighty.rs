pub struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split('-')
            .map(|s| format!("{:b}", s.parse::<u16>().unwrap()))
            .collect::<Vec<_>>()
            .join("-")
    }
}
