pub struct Solution;

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    index_to_number: RefCell<HashMap<i32, i32>>,
    number_to_indices: RefCell<HashMap<i32, BTreeSet<i32>>>,
}

impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            index_to_number: RefCell::new(HashMap::new()),
            number_to_indices: RefCell::new(HashMap::new()),
        }
    }

    fn change(&self, index: i32, number: i32) {
        let mut index_to_number = self.index_to_number.borrow_mut();
        let mut number_to_indices = self.number_to_indices.borrow_mut();

        if let Some(old_number) = index_to_number.get(&index) {
            if let Some(indices) = number_to_indices.get_mut(old_number) {
                indices.remove(&index);
            }
        }

        index_to_number.insert(index, number);

        number_to_indices
            .entry(number)
            .or_insert_with(BTreeSet::new)
            .insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        let number_to_indices = self.number_to_indices.borrow();

        if let Some(indices) = number_to_indices.get(&number) {
            if let Some(min_index) = indices.iter().next() {
                return *min_index;
            }
        }

        -1
    }
}
