pub struct Solution;

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let bytes = s.as_bytes();

        // 每个字符出现的位置
        let mut pos = vec![Vec::<usize>::new(); 26];
        for (i, &b) in bytes.iter().enumerate() {
            pos[(b - b'a') as usize].push(i);
        }

        // g[x]：字符 x 的最小区间内出现了哪些其他字符
        let mut graph = vec![Vec::<usize>::new(); 26];

        for i in 0..26 {
            if pos[i].is_empty() {
                continue;
            }

            let left = pos[i][0];
            let right = *pos[i].last().unwrap();

            for j in 0..26 {
                if i == j || pos[j].is_empty() {
                    continue;
                }

                // 等价于 Go 的 sort.SearchInts(pos[j], left)
                let k = pos[j].partition_point(|&x| x < left);

                if k < pos[j].len() && pos[j][k] <= right {
                    graph[i].push(j);
                }
            }
        }

        let mut intervals = Vec::new();

        for start in 0..26 {
            if pos[start].is_empty() {
                continue;
            }

            let mut visited = [false; 26];
            let mut stack = vec![start];

            let mut left = bytes.len();
            let mut right = 0;

            while let Some(x) = stack.pop() {
                if visited[x] {
                    continue;
                }

                visited[x] = true;

                left = left.min(pos[x][0]);
                right = right.max(*pos[x].last().unwrap());

                for &y in &graph[x] {
                    if !visited[y] {
                        stack.push(y);
                    }
                }
            }

            intervals.push((left, right));
        }

        // 按右端点排序，然后贪心选择不相交区间
        intervals.sort_unstable_by_key(|&(_, right)| right);

        let mut ans = Vec::new();
        let mut prev_right = None;

        for (left, right) in intervals {
            if prev_right.is_none_or(|r| left > r) {
                ans.push(s[left..=right].to_string());
                prev_right = Some(right);
            }
        }

        ans
    }
}