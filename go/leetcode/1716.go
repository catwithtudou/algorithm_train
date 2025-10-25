package leetcode

func totalMoney(n int) int {
	week := 7
	w, r := n/week, n%week
	return (w*week*(w+week) + r*(w*2+r+1)) / 2
}
