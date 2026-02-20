pub struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        if s.len() <= 2 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut sub_strings: Vec<String> = Vec::new();
        let mut diff = 0i32;
        let mut start = 0usize;

        for (i, &ch) in chars.iter().enumerate() {
            if ch == '1' {
                diff += 1;
            } else {
                diff -= 1;
                if diff == 0 {
                    let inner: String = chars[start + 1..i].iter().collect();
                    let inner_processed = Self::make_largest_special(inner);
                    sub_strings.push(format!("1{}0", inner_processed));
                    start = i + 1;
                }
            }
        }

        sub_strings.sort_by(|a, b| b.cmp(a));
        sub_strings.join("")
    }
}
