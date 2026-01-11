pub struct Solution;

impl Solution {
    fn largest_rectangle_area(heights: &[i32]) -> i32 {
        let mut st = vec![-1];
        let mut ans = 0;
        for (right, &h) in heights.iter().enumerate() {
            let right = right as i32;
            while st.len() > 1 && heights[*st.last().unwrap() as usize] >= h {
                let i = st.pop().unwrap() as usize;
                let left = *st.last().unwrap();
                ans = ans.max(heights[i] * (right - left - 1));
            }
            st.push(right);
        }
        ans
    }

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        let mut heights = vec![0; matrix[0].len() + 1];
        for row in matrix {
            for (j, &c) in row.iter().enumerate() {
                heights[j] = if c == '0' { 0 } else { heights[j] + 1 };
            }
            ans = ans.max(Self::largest_rectangle_area(&heights));
        }
        ans
    }
}
