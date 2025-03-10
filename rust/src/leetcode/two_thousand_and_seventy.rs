pub struct Solution;

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable_by_key(|item| item[0]);

        for i in 1..items.len() {
            items[i][1] = items[i][1].max(items[i - 1][1]);
        }

        queries
            .into_iter()
            .map(|q| {
                let j = items.partition_point(|item| item[0] <= q);
                if j > 0 {
                    items[j - 1][1]
                } else {
                    0
                }
            })
            .collect()
    }
}
