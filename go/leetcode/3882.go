package leetcode

import "math"

func minCost3882(grid [][]int) int {
	m, n := len(grid), len(grid[0])

	orAll := 0

	for _, row := range grid {
		for _, x := range row {
			orAll |= x
		}
	}

	vis := make([][][]bool, m)
	for i := range vis {
		vis[i] = make([][]bool, n)
		for j := range vis[i] {
			vis[i][j] = make([]bool, orAll+1)
		}
	}

	ans := math.MaxInt

	var dfs func(int, int, int)

	dfs = func(i, j, xor int) {
		if ans == 0 || i < 0 || j < 0 || vis[i][j][xor] {
			return
		}

		vis[i][j][xor] = true

		xor ^= grid[i][j]

		if i == 0 && j == 0 {
			ans = min(ans, xor)
			return
		}

		dfs(i-1, j, xor)
		dfs(i, j-1, xor)
	}

	dfs(m-1, n-1, 0)

	return ans
}
