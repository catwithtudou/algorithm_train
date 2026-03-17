package leetcode

import "golang.org/x/exp/slices"

func largestSubmatrix(matrix [][]int) (ans int) {
	n := len(matrix[0])
	heights := make([]int, n)

	for _, row := range matrix {
		for j, x := range row {
			if x == 0 {
				heights[j] = 0
			} else {
				heights[j]++
			}
		}

		hs := slices.Clone(heights)

		slices.Sort(hs)

		for i, h := range hs {
			ans = max(ans, h*(n-i))
		}
	}

	return
}
