pub struct Solution;

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let mut ans = 0;
        let mut f = vec![0; budget as usize + 1];
        let mut min_p = i32::MAX;

        for i in 0..items.len() {
            let fac = items[i][0];
            let pri = items[i][1];
            min_p = min_p.min(pri);

            let mut cnt = 0;
            for i in 0..items.len() {
                if items[i][0] % fac == 0 {
                    cnt += 1;
                }
            }

            for j in (pri..=budget).rev() {
                f[j as usize] = f[j as usize].max(f[(j - pri) as usize] + cnt);
            }
        }

        for i in 0..f.len() {
            ans = ans.max(f[i]+(budget-i as i32)/min_p);
        }

        ans
    }
}
