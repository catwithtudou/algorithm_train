pub struct Solution;

use std::collections::BTreeSet;
#[allow(dead_code)]
impl Solution {
    pub fn closest_room(mut rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        rooms.sort_unstable_by_key(|r| -r[1]);

        let  q = queries.len();
        let mut query_ids = (0..q).collect::<Vec<_>>();
        query_ids.sort_unstable_by_key(|&i| -queries[i][1]);

        let mut ans = vec![-1; q];
        let mut room_ids = BTreeSet::new();
        let mut j = 0;
        for i in query_ids {
            let (prefer_id, min_size) = (queries[i][0], queries[i][1]);

            while j < rooms.len() && rooms[j][1] >= min_size {
                room_ids.insert(rooms[j][0]);
                j += 1;
            }

            let mut diff = i32::MAX;
            if let Some(&prev) = room_ids.range(..prefer_id).next_back() {
                diff = prefer_id - prev;
                ans[i] = prev;
            }
            if let Some(&next) = room_ids.range(prefer_id..).next() {
                if next - prefer_id < diff {
                    ans[i] = next;
                }
            }
        }
        ans
    }
}
