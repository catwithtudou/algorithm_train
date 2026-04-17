pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut indices: HashMap<i32, Vec<i32>> = HashMap::new();

        for (i, &x) in nums.iter().enumerate() {
            indices.entry(x).or_default().push(i as i32);
        }

        for positions in indices.values_mut() {
            let first = positions[0];
            let last = positions[positions.len() - 1];

            let mut extended = Vec::with_capacity(positions.len() + 2);
            extended.push(last - n);
            extended.extend(positions.iter().copied());
            extended.push(first + n);

            *positions = extended;
        }

        queries
            .into_iter()
            .map(|i| {
                let i = i as i32;
                let positions = &indices[&nums[i as usize]];

                if positions.len() == 3 {
                    -1
                } else {
                    let j = positions.partition_point(|&x| x < i);
                    (i - positions[j - 1]).min(positions[j + 1] - i)
                }
            })
            .collect()
    }
}