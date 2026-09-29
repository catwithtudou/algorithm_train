package leetcode

func hasValidPath2267(grid [][]byte) bool {
	m, n := len(grid), len(grid[0])
	if (m+n)%2 == 0 || grid[0][0] == ')' || grid[m-1][n-1] == '(' {
		return false
	}

	vis := make([][][]bool, m)
	for i := range vis {
		vis[i] = make([][]bool, n)
		for j := range vis[i] {
			vis[i][j] = make([]bool, (m+n+1)/2)
		}
	}

	var dfs func(int, int, int) bool

	dfs = func(x, y, c int) bool {
		if c > m-x+n-y-1 {
			return false
		}
		if x == m-1 && y == n-1 {
			return c == 1
		}

		if vis[x][y][c] {
			return false
		}
		vis[x][y][c] = true

		if grid[x][y] == '(' {
			c++
		} else if c--; c < 0 {
			return false
		}

		return x < m-1 && dfs(x+1, y, c) || y < n-1 && dfs(x, y+1, c)
	}

	return dfs(0, 0, 0)
}
