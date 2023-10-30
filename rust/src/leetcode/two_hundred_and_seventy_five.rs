pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut left = 1;
        let mut right = n;
        while left <= right {
            let mid = (left + right) / 2;
            if citations[n - mid] >= mid as i32 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        right as i32
    }
}

#[cfg(test)]
mod two_hundred_and_seventy_five_test {
    use super::*;

    #[test]
    fn test_two_hundred_and_seventy_five() {
        assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
        assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6, 7]), 3);
    }
}