package leetcode

import "strings"

func solveNQueens(n int) (ans [][]string) {
	queen := make([]int, n)
	col := make([]bool, n)
	dig1 := make([]bool, 2*n-1)
	dig2 := make([]bool, 2*n-1)
	var dfs func(int)
	dfs = func(r int) {
		if r == n {
			board := make([]string, n)
			for i, c := range queen {
				board[i] = strings.Repeat(".", c) + "Q" + strings.Repeat(".", n-c-1)
			}
			ans = append(ans, board)
			return
		}
		for c, ok := range col {
			rc := r - c + n - 1
			if !ok && !dig1[r+c] && !dig2[rc] {
				queen[r] = c
				col[c], dig1[r+c], dig2[rc] = true, true, true
				dfs(r + 1)
				col[c], dig1[r+c], dig2[rc] = false, false, false
			}
		}

	}
	dfs(0)
	return
}
