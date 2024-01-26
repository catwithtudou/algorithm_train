pub struct Solution;


impl Solution {
    pub fn max_number_of_alloys(n: i32, k: i32, budget: i32, composition: Vec<Vec<i32>>, stock: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, *stock.iter().min().unwrap() as i32 + budget / n);
        let check = |num| -> bool{
            composition.iter().any(|comp| {
                let mut spend = 0i64;
                for (j, &v) in comp.iter().enumerate() {
                    if v as i64 * num > stock[j] as i64 {
                        spend += (v as i64 * num - stock[j] as i64) * cost[j] as i64;
                    }
                }
                spend <= budget as i64
            })
        };

        let mut ans = 0;
        while left <= right {
            let mid = (left + right + 1) / 2;
            if check(mid as i64) {
                left = mid + 1;
                ans = mid;
            } else {
                right = mid - 1;
            }
        }

        ans
    }
}