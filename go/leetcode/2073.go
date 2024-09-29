package leetcode

func timeRequiredToBuy(tickets []int, k int) int {
	tk := tickets[k]
	ans := 0
	for i, v := range tickets {
		if i <= k {
			ans += min(tk, v)
		} else {
			ans += min(tk-1, v)
		}
	}
	return ans
}
