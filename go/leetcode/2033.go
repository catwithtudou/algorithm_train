package leetcode

import "golang.org/x/exp/slices"

func minOperations2033(grid [][]int, x int) int {
	k := len(grid) * len(grid[0])
	a := make([]int, 0, k)
	traget := grid[0][0] % x

	for _, row := range grid {
		for _, v := range row {
			if v%x != traget {
				return -1
			}
		}
		a = append(a, row...)
	}

	slices.Sort(a)
	median := a[k/2]

	ans := 0
	for _, v := range a {
		ans += abs(v - median)
	}

	return ans / x
}
