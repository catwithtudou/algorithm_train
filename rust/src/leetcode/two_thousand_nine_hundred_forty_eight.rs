pub struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();

        let mut ans = vec![0; n];

        let mut arr: Vec<(i32, usize)> =
            nums.into_iter().enumerate().map(|(i, x)| (x, i)).collect();

        arr.sort_by_key(|p| p.0);

        let values: Vec<i32> = arr.iter().map(|p| p.0).collect();
        let include: Vec<usize> = arr.iter().map(|p| p.1).collect();

        let mut i = 0;
        while i < n {
            let start = i;

            let mut group_indices = Vec::new();

            let mut group_values = Vec::new();

            while i < n && (i == start || values[i] - values[i - 1] <= limit) {
                group_indices.push(include[i]);
                group_values.push(values[i]);
                i += 1;
            }

            group_indices.sort();

            for (index, value) in group_indices.into_iter().zip(group_values.into_iter()) {
                ans[index] = value;
            }
        }

        ans
    }
}
