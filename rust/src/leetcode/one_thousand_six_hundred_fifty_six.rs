pub struct Solution;

struct OrderedStream {
    str_list: Vec<String>,
    ptr: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            str_list: vec![String::new(); (n + 1) as usize],
            ptr: 1,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.str_list[id_key as usize] = value;
        let start = self.ptr;
        while self.ptr < self.str_list.len() && !self.str_list[self.ptr].is_empty() {
            self.ptr += 1;
        }
        self.str_list[start..self.ptr].to_vec()
    }
}