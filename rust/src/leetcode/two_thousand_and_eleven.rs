pub struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut sub_num = 0;
        let mut add_num = 0;

        for op in operations {
            if op.as_bytes()[1] == b'-' {
                sub_num += 1;
            } else {
                add_num += 1;
            }
        }

        add_num - sub_num
    }
}