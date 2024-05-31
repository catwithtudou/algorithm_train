package leetcode

func findMissingAndRepeatedValues(grid [][]int) []int {
	n := len(grid)
	cnt := make([]int, n*n+1)
	for _, row := range grid {
		for _, v := range row {
			cnt[v]++
		}
	}

	ans := make([]int, 2)
	for i := 1; i <= n*n; i++ {
		if cnt[i] == 2 {
			ans[0] = i
		} else if cnt[i] == 0 {
			ans[1] = i
		}
	}

	return ans
}
