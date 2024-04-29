package leetcode

import "golang.org/x/exp/slices"

func diagonalSort(mat [][]int) [][]int {
	m, n := len(mat), len(mat[0])
	arr := make([]int, min(m, n))
	for k := n - 1; k < m; k++ {
		temp := arr[:0]
		left := max(k, 0)
		right := min(m, k+n)
		for i := left; i < right; i++ {
			temp = append(temp, mat[i][i-k])
		}
		slices.Sort(temp)
		for i := left; i < right; i++ {
			mat[i][i-k] = temp[i-left]
		}

	}
	return mat
}
