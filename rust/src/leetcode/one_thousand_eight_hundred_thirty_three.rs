pub struct Solution;

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, mut coins: i32) -> i32 {
        let mx = *costs.iter().max().unwrap();

        let mut cnt = vec![0; mx as usize + 1];

        for x in costs {
            cnt[x as usize] += 1;
        }

        let mut ans = 0;

        for cost in 1..=mx {
            if coins < cost {
                break;
            }

            let num = cnt[cost as usize].min(coins / cost);

            coins -= cost * num;
            ans += num;
        }

        ans
    }
}
