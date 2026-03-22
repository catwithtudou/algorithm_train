package leetcode

import "golang.org/x/exp/slices"

func findRotation(mat [][]int, target [][]int) bool {
	rotate := func(matrix [][]int) {
		n := len(matrix)
		for i, row := range matrix {
			for j := i + 1; j < n; j++ {
				row[j], matrix[j][i] = matrix[j][i], row[j]
			}
			slices.Reverse(row)
		}
	}

	for range 4 {
		if slices.EqualFunc(mat, target, slices.Equal[[]int]) {
			return true
		}
		rotate(mat)
	}

	return false
}
