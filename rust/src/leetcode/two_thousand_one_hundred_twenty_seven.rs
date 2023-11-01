use std::collections::VecDeque;

pub struct Solution;


impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        let mut deg = vec![0; n];
        for &f in &favorite {
            deg[f as usize] += 1;
        }


        // 与 go 解决不同使用反图+dfs的方式计算深度
        let mut rg = vec![vec![]; n];
        let mut q = VecDeque::new();
        for (i, &d) in deg.iter().enumerate() {
            if d == 0 {
                q.push_back(i);
            }
        }

        // 拓扑排序
        while let Some(x) = q.pop_front() {
            let y = favorite[x] as usize;
            rg[y].push(x);
            deg[y] -= 1;
            if deg[y] == 0 {
                q.push_back(y)
            }
        }

        fn rdfs(x: usize, rg: &Vec<Vec<usize>>) -> i32 {
            let mut max_depth = 1;
            for &son in &rg[x] {
                max_depth = max_depth.max(rdfs(son, rg) + 1);
            }
            max_depth
        }

        let mut max_ring_size = 0;
        let mut sum_chain_size = 0;
        for i in 0..n {
            if deg[i] == 0 {
                continue;
            }

            deg[i] = 0;
            let mut ring_size = 1;
            let mut x = favorite[i] as usize;
            while x != i {
                deg[x] = 0;
                ring_size += 1;
                x = favorite[x] as usize;
            }

            if ring_size != 2 {
                max_ring_size = max_ring_size.max(ring_size);
            } else {
                sum_chain_size += rdfs(i, &rg) + rdfs(favorite[i] as usize, &rg);
            }
        }

        sum_chain_size.max(max_ring_size)
    }
}


#[cfg(test)]
mod two_thousand_one_hundred_twenty_seven_test {
    use super::*;

    #[test]
    fn two_thousand_one_hundred_twenty_seven() {
        assert_eq!(Solution::maximum_invitations(vec![2,2,1,2]),3);
        assert_eq!(Solution::maximum_invitations(vec![1,2,0]),3);
    }
}


