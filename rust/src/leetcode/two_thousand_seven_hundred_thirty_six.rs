pub struct Solution;

impl Solution {
    pub fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pair_list: Vec<(i32, i32)> = nums1.into_iter().zip(nums2.into_iter()).collect();
        pair_list.sort_by(|x, y| y.0.cmp(&x.0));

        let mut q_idx: Vec<usize> = (0..queries.len()).collect();
        q_idx.sort_by(|&i, &j| queries[j][0].cmp(&queries[i][0]));

        let mut ans = vec![-1; queries.len()];
        let mut st: Vec<(i32, i32)> = Vec::new();
        let mut j = 0;
        for &i in &q_idx {
            let (x, y) = (queries[i][0], queries[i][1]);
            while j < pair_list.len() && pair_list[j].0 >= x {
                while !st.is_empty() && st.last().unwrap().1 <= pair_list[j].0 + pair_list[j].1 {
                    st.pop();
                }
                if st.is_empty() || pair_list[j].1 > st.last().unwrap().0 {
                    st.push((pair_list[j].1, pair_list[j].0 + pair_list[j].1))
                }
                j += 1;
            }

            let p = st.partition_point(|&p| p.0 < y);
            if p < st.len() {
                ans[i] = st[p].1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod two_thousand_seven_hundred_thirty_six_test {
    use super::*;

    #[test]
    fn test_maximum_sum_queries() {
        assert_eq!(Solution::maximum_sum_queries(vec![4, 3, 1, 2], vec![2, 4, 9, 5], vec![vec![4, 1], vec![1, 3], vec![2, 5]]),
                   vec![6, 10, 7])
    }
}