package leetcode

var dirsIdx = []struct{ x, y int }{{0, -1}, {0, 1}, {-1, 0}, {1, 0}}

func containsCycle(grid [][]byte) bool {
	m, n := len(grid), len(grid[0])
	vis := make([][]bool, m)
	for i := range vis {
		vis[i] = make([]bool, n)
	}

	var dfs func(int, int, int, int) bool
	dfs = func(x, y, px, py int) bool {
		vis[x][y] = true
		for _, d := range dirsIdx {
			i, j := x+d.x, y+d.y
			if (i != px || j != py) && 0 <= i && i < m && 0 <= j && j < n && grid[i][j] == grid[x][y] && (vis[i][j] || dfs(i, j, x, y)) {
				return true
			}
		}
		return false
	}

	for i, row := range vis {
		for j, b := range row {
			if !b && dfs(i, j, -1, -1) {
				return true
			}
		}
	}

	return false
}
