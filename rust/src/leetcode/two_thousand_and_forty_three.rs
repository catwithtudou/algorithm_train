pub struct Solution;

struct Bank {
    b: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { b: balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let account1 = account1 as usize;
        let account2 = account2 as usize;
        if account1 > self.b.len() || account2 > self.b.len() || self.b[account1 - 1] < money {
            return false;
        }
        self.b[account1 - 1] -= money;
        self.b[account2 - 1] += money;
        return true;
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize;
        if account > self.b.len() {
            return false;
        }
        self.b[account - 1] += money;
        return true;
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize;
        if account > self.b.len() || self.b[account - 1] < money {
            return false;
        }
        self.b[account - 1] -= money;
        return true;
    }
}
