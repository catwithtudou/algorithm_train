pub struct Solution;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut cnt = [0; 10];
        for d in digits {
            cnt[d as usize] += 1;
        }

        fn dfs(i: usize, x: i32, cnt: &mut [i32; 10], ans: &mut Vec<i32>) {
            if i == 3 {
                ans.push(x);
                return;
            }

            for d in 0..10 {
                if cnt[d] > 0 && (i == 0 && d > 0 || i == 1 || i == 2 && d % 2 == 0) {
                    cnt[d] -= 1;
                    dfs(i + 1, x * 10 + d as i32, cnt, ans);
                    cnt[d] += 1;
                }
            }
        }

        let mut ans = vec![];
        dfs(0, 0, &mut cnt, &mut ans);
        ans
    }
}
