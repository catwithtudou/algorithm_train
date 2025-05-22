use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        queries.sort_by_key(|q| q[0]);

        let mut heap = BinaryHeap::new();
        let mut diff = vec![0; nums.len() + 1];
        let (mut sum_d, mut j) = (0, 0);

        for (i, &x) in nums.iter().enumerate() {
            sum_d += diff[i];

            while j < queries.len() && queries[j][0] <= i as i32 {
                heap.push(queries[j][1]);
                j += 1;
            }

            while sum_d < x && !heap.is_empty() && *heap.peek().unwrap() >= i as i32 {
                sum_d += 1;
                if let Some(val) = heap.pop() {
                    diff[(val + 1) as usize] -= 1;
                }
            }
            if sum_d < x {
                return -1;
            }
        }

        heap.len() as i32
    }
}
