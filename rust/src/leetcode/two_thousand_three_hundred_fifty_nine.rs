pub struct Solution;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();

        let calc_dis = |start: i32| -> Vec<usize> {
            let (mut dis, mut x, mut d) = (vec![n; n], start, 0);

            while x >= 0 && dis[x as usize] == n {
                dis[x as usize] = d;
                d += 1;
                x = edges[x as usize];
            }

            dis
        };

        let (dis1, dis2) = (calc_dis(node1), calc_dis(node2));

        let (mut min_dis, mut ans) = (n, -1);

        for (i, &d1) in dis1.iter().enumerate() {
            let d = d1.max(dis2[i]);
            if d < min_dis {
                min_dis = d;
                ans = i as i32;
            }
        }

        ans
    }
}
