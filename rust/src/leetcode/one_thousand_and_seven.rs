pub struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let min_rot = |target: i32| -> i32 {
            let (mut to_top, mut to_bottom) = (0, 0);
            for (&x, &y) in tops.iter().zip(bottoms.iter()) {
                if x != target && y != target {
                    return i32::MAX;
                }
                if x != target {
                    to_top += 1;
                } else if y != target {
                    to_bottom += 1;
                }
            }
            to_top.min(to_bottom)
        };

        let ans = min_rot(tops[0]).min(min_rot(bottoms[0]));
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
