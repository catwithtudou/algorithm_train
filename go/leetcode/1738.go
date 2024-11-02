package leetcode

import "golang.org/x/exp/slices"

func kthLargestValue(matrix [][]int, k int) int {
	m, n := len(matrix), len(matrix[0])
	a := make([]int, 0, m*n)
	s := make([][]int, m+1)
	for i := range s {
		s[i] = make([]int, n+1)
	}
	for i, row := range matrix {
		for j, col := range row {
			s[i+1][j+1] = s[i+1][j] ^ s[i][j+1] ^ s[i][j] ^ col
		}
		a = append(a, s[i+1][1:]...)
	}
	slices.Sort(a)
	return a[len(a)-k]
}
