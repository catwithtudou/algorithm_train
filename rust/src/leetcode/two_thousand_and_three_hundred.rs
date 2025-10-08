pub struct Solution;

impl Solution {
    pub fn successful_pairs(mut spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable(); // 升序
        let max_p: i64 = potions[potions.len() - 1] as i64;
        for x in spells.iter_mut() {
            let target: i64 = (success - 1) / *x as i64;
            if target >= max_p {
                *x = 0;
                continue;
            }
            let j = potions.partition_point(|&x| x <= target as i32);
            *x = (potions.len() - j) as i32;
        }
        spells
    }

    pub fn successful_pairs_others(
        mut spells: Vec<i32>,
        mut potions: Vec<i32>,
        success: i64,
    ) -> Vec<i32> {
        potions.sort_unstable();
        let last = potions[potions.len() - 1] as i64;
        for x in spells.iter_mut() {
            let target = success as f64 / *x as f64;
            let j = potions.partition_point(|&x| (x as f64) < target);
            *x = (potions.len() - j) as i32;
        }
        spells
    }
}

#[cfg(test)]
mod two_thousand_and_three_hundred_test {
    use super::*;

    #[test]
    fn test_successful_pairs() {
        assert_eq!(
            Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
    }
}
