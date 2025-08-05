pub struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let n = fruits.len();
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..n {
                if fruits[i] <= baskets[j] {
                    baskets[j] = 0;
                    cnt += 1;
                    break;
                }
            }
        }
        (n - cnt) as i32
    }
}
