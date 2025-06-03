pub struct Solution;

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut has_key = status;
        let mut has_box = vec![false; has_key.len()];
        for &x in &initial_boxes {
            has_box[x as usize] = true;
        }

        fn dfs(
            x: usize,
            candies: &Vec<i32>,
            keys: &[Vec<i32>],
            contained_boxes: &[Vec<i32>],
            has_key: &mut Vec<i32>,
            has_box: &mut Vec<bool>,
            ans: &mut i32,
        ) {
            *ans += candies[x];
            has_box[x] = false;

            for &y in &keys[x] {
                has_key[y as usize] = 1;
                if has_box[y as usize] {
                    dfs(
                        y as usize,
                        candies,
                        keys,
                        contained_boxes,
                        has_key,
                        has_box,
                        ans,
                    );
                }
            }

            for &y in &contained_boxes[x] {
                has_box[y as usize] = true;
                if has_key[y as usize] == 1 {
                    dfs(
                        y as usize,
                        candies,
                        keys,
                        contained_boxes,
                        has_key,
                        has_box,
                        ans,
                    );
                }
            }
        }

        let mut ans = 0;
        for x in initial_boxes {
            let x = x as usize;
            if has_box[x] && has_key[x] == 1 {
                dfs(
                    x,
                    &candies,
                    &keys,
                    &contained_boxes,
                    &mut has_key,
                    &mut has_box,
                    &mut ans,
                );
            }
        }

        ans
    }
}
