package leetcode

type Bank []int64

func ConstructorBank(balance []int64) Bank {
	return balance
}

func (b Bank) Transfer(account1 int, account2 int, money int64) bool {
	if account1 > len(b) || account2 > len(b) || b[account1-1] < money {
		return false
	}
	b[account1-1] -= money
	b[account2-1] += money
	return true
}

func (b Bank) Deposit(account int, money int64) bool {
	if account > len(b) {
		return false
	}
	b[account-1] += money
	return true
}

func (b Bank) Withdraw(account int, money int64) bool {
	if account > len(b) || b[account-1] < money {
		return false
	}
	b[account-1] -= money
	return true

}

/**
 * Your Bank object will be instantiated and called as such:
 * obj := Constructor(balance);
 * param_1 := obj.Transfer(account1,account2,money);
 * param_2 := obj.Deposit(account,money);
 * param_3 := obj.Withdraw(account,money);
 */
