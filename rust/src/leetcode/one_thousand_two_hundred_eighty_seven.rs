pub struct Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let m = arr.len() / 4;
        for i in [m, 2 * m + 1] {
            let x = arr[i];
            let j = arr.partition_point(|&y| y < x);
            if arr[j + m] == x {
                return x;
            }
        }
        arr[3 * m + 2]
    }
}
