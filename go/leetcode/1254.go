package leetcode

func closedIsland(grid [][]int) (ans int) {

	m, n := len(grid), len(grid[0])

	if m < 3 || n < 3 {
		return
	}

	var dfs func(int, int)
	dfs = func(x, y int) {
		if x < 0 || x >= m || y < 0 || y >= n || grid[x][y] != 0 {
			return
		}

		grid[x][y] = 1
		dfs(x-1, y)
		dfs(x+1, y)
		dfs(x, y-1)
		dfs(x, y+1)
	}

	for i := 0; i < m; i++ {
		step := 1
		if 0 < i && i < m-1 {
			step = n - 1
		}
		for j := 0; j < n; j += step {
			dfs(i, j)
		}
	}

	for i := 1; i < m-1; i++ {
		for j := 1; j < n-1; j++ {
			if grid[i][j] == 0 {
				ans++
				dfs(i, j)
			}
		}
	}

	return
}
