pub struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let n = fruits.len();

        let mut left = fruits
            .binary_search_by(|fruit| {
                if fruit[0] < start_pos - k {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .unwrap_or_else(|i| i);

        let mut right = left;
        let mut s = 0;

        while right < n && fruits[right][0] <= start_pos {
            s += fruits[right][1];
            right += 1;
        }

        let mut ans = s;

        while right < n && fruits[right][0] <= start_pos + k {
            s += fruits[right][1];

            while left < right
                && (fruits[right][0] * 2 - fruits[left][0] - start_pos > k
                    && fruits[right][0] - fruits[left][0] * 2 + start_pos > k)
            {
                s -= fruits[left][1];
                left += 1;
            }

            ans = ans.max(s);
            right += 1;
        }

        ans
    }
}
