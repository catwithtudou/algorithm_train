pub struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut idx = (0..arr.len()).collect::<Vec<_>>();
        idx.sort_unstable_by_key(|&i| arr[i]);

        let mut ans = 0;
        for &j in &idx {
            let y = arr[j];
            let mut left = vec![];
            for &i in &idx {
                if i < j && (arr[i] - y).abs() <= a {
                    left.push(arr[i]);
                }
            }

            let mut right = vec![];
            for &k in &idx {
                if j < k && (arr[k] - y).abs() <= b {
                    right.push(arr[k]);
                }
            }

            let (mut k1, mut k2) = (0, 0);
            for x in left {
                while k2 < right.len() && right[k2] <= x + c {
                    k2 += 1;
                }
                while k1 < right.len() && right[k1] < x - c {
                    k1 += 1;
                }
                ans += k2 - k1;
            }
        }

        ans as i32
    }
}
