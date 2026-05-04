package leetcode

import "slices"

func rotate48(matrix [][]int) {
	n := len(matrix)

	for i := range n {
		for j := range i {
			matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]
		}
	}

	for _, row := range matrix {
		slices.Reverse(row)
	}
}
