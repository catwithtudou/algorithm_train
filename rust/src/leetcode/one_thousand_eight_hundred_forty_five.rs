use std::collections::BinaryHeap;

pub struct Solution;
#[allow(dead_code)]
struct SeatManager {
    seats:i32,
    available:BinaryHeap<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl SeatManager {

    fn new(_: i32) -> Self {
        Self{seats:0, available:BinaryHeap::new()}
    }

    fn reserve(&mut self) -> i32 {
        if let Some(seat) =self.available.pop(){
            -seat
        }else{
            self.seats+=1;
            self.seats
        }
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.available.push(-seat_number);
    }
}

