package leetcode

func minIncrements(n int, cost []int) int {
	ans := 0
	for i := n / 2; i > 0; i-- {
		left, right := cost[i*2-1], cost[i*2]
		if left > right {
			left, right = right, left
		}
		ans += right - left
		cost[i-1] += right
	}

	return ans
}
