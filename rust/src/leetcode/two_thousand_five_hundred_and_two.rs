pub struct Solution;

struct Allocator {
    list: Vec<i32>,
}

impl Allocator {
    fn new(n: i32) -> Self {
        Self {
            list: vec![0; n as usize],
        }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut free = 0;
        for (i, &id) in self.list.iter().enumerate() {
            if id > 0 {
                free = 0;
                continue;
            }
            free += 1;
            if free == size {
                for j in (i - size as usize + 1)..=i {
                    self.list[j] = m_id;
                }
                return i as i32 - size + 1;
            }
        }
        -1
    }

    fn free_memory(&mut self, m_id: i32) -> i32 {
        let mut ans = 0;
        for i in 0..self.list.len() {
            if self.list[i] == m_id {
                ans += 1;
                self.list[i] = 0;
            }
        }
        ans
    }
}
