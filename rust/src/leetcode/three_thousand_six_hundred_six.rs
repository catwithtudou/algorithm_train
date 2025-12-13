pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let business_line_to_category: HashMap<&str, usize> = HashMap::from([
            ("electronics", 0),
            ("grocery", 1),
            ("pharmacy", 2),
            ("restaurant", 3),
        ]);

        let mut groups: [Vec<String>; 4] = std::array::from_fn(|_| Vec::new());

        for i in 0..code.len() {
            if let Some(&category) = business_line_to_category.get(business_line[i].as_str()) {
                if is_active[i] && Self::is_valid_code(&code[i]) {
                    groups[category].push(code[i].clone());
                }
            }
        }

        let mut ans = Vec::new();
        for g in groups.iter_mut() {
            g.sort();
            ans.extend(g.drain(..));
        }

        ans
    }

    fn is_valid_code(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        s.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
    }
}
