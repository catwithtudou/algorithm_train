package leetcode

import "math"

func pathsWithMaxScore(board []string) []int {
	m, n := len(board), len(board[0])
	maxSums := make([][]int, m+1)
	ways := make([][]int, m+1)
	for i := range maxSums {
		maxSums[i] = make([]int, n+1)
		for j := range maxSums[i] {
			maxSums[i][j] = math.MinInt
		}
		ways[i] = make([]int, n+1)
	}

	maxSums[0][0] = 0
	ways[0][0] = 1

	for i, row := range board {
		for j, ch := range row {
			if ch == 'X' {
				continue
			}

			maxSums[i+1][j+1] = max(maxSums[i][j], max(maxSums[i][j+1], maxSums[i+1][j]))
			s := maxSums[i+1][j+1]
			w := 0
			if maxSums[i][j] == s {
				w += ways[i][j]
			}
			if maxSums[i+1][j] == s {
				w += ways[i+1][j]
			}
			if maxSums[i][j+1] == s {
				w += ways[i][j+1]
			}
			ways[i+1][j+1] = w % int(1e9+7)
			if '1' <= ch && ch <= '9' {
				maxSums[i+1][j+1] += int(ch - '0')
			}
		}
	}

	if maxSums[m][n] < 0 {
		return []int{0, 0}
	}

	return []int{maxSums[m][n], ways[m][n]}
}
