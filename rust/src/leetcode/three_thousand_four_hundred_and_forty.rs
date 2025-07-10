pub struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut ans = 0;

        // 计算空位长度的函数
        let get = |i: usize| -> i32 {
            if i == 0 {
                start_time[0]
            } else if i == n {
                event_time - end_time[n - 1]
            } else {
                start_time[i] - end_time[i - 1]
            }
        };

        // 有 n+1 个空位，计算前三大的空位在哪
        let mut a = 0usize;
        let mut b: Option<usize> = None;
        let mut c: Option<usize> = None;

        for i in 1..=n {
            let sz = get(i);
            if sz > get(a) {
                c = b;
                b = Some(a);
                a = i;
            } else if b.is_none() || sz > get(b.unwrap()) {
                c = b;
                b = Some(i);
            } else if c.is_none() || sz > get(c.unwrap()) {
                c = Some(i);
            }
        }

        // 枚举会议
        for i in 0..n {
            let sz = end_time[i] - start_time[i];

            // 检查是否可以移出去
            let can_move = (i != a && i + 1 != a && sz <= get(a))
                || (b.is_some() && i != b.unwrap() && i + 1 != b.unwrap() && sz <= get(b.unwrap()))
                || (c.is_some() && sz <= get(c.unwrap()));

            if can_move {
                ans = ans.max(get(i) + sz + get(i + 1));
            } else {
                ans = ans.max(get(i) + get(i + 1));
            }
        }

        ans
    }
}
