pub struct Solution;

impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, mut queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut pos = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if num == x {
                pos.push(i);
            }
        }

        for i in 0..queries.len() {
            if queries[i] > pos.len() as i32 {
                queries[i] = -1;
            } else {
                queries[i] = pos[(queries[i] - 1) as usize] as i32;
            }
        }

        queries
    }
}
