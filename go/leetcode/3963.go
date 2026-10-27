package leetcode

import "strings"

func createGrid(m int, n int) []string {
	ans := make([]string, m)
	ans[0] = strings.Repeat(".", n)
	row := strings.Repeat("#", n-1) + "."
	for i := 1; i < m; i++ {
		ans[i] = row
	}
	return ans
}
