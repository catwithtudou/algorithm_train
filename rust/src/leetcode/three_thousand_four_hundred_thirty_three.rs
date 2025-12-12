pub struct Solution;

impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let n = number_of_users as usize;
        let mut ans = vec![0; n];
        let mut es = vec![];
        let mut all = 0;
        for e in events {
            let cur_t = e[1].parse::<i32>().unwrap();
            let mention = &e[2];
            if e[0].as_bytes()[0] == b'O' {
                let i = mention.parse::<usize>().unwrap();
                es.push((cur_t, 1, i));
                es.push((cur_t + 60, -1, i));
                continue;
            }
            if mention.as_bytes()[0] == b'A' {
                all += 1;
            } else if mention.as_bytes()[0] == b'H' {
                all += 1;
                es.push((cur_t, 2, 0));
            } else {
                for s in mention.split(' ') {
                    ans[s[2..].parse::<usize>().unwrap()] += 1;
                }
            }
        }

        es.sort_unstable();

        let mut here = 0;
        for (_, type_, id) in es {
            if type_ == 2 {
                here += 1;
            } else {
                ans[id] += type_ * here;
            }
        }

        for cnt in ans.iter_mut() {
            *cnt += all;
        }

        ans
    }
}
