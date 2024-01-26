package leetcode

func maxNumberOfAlloys(n int, k int, budget int, composition [][]int, stock []int, cost []int) int {
	left, right := 0, int(2e8)
	for left < right {
		mid := (left + right + 1) >> 1
		valid := false
		for i := 0; i < k; i++ {
			spend := 0
			for j := 0; j < n; j++ {
				spend += max(composition[i][j]*mid-stock[j], 0) * cost[j]
			}
			if spend <= budget {
				valid = true
				break
			}
		}
		if valid {
			left = mid
		} else {
			right = mid - 1
		}
	}
	return left
}
