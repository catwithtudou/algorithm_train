package leetcode

import "sort"

var dirsSwimInWater = []struct{ x, y int }{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

func swimInWater(grid [][]int) int {
	n := len(grid)
	vis := make([][]int, n)
	for i := range vis {
		vis[i] = make([]int, n)
		for j := range vis[i] {
			vis[i][j] = -1
		}
	}

	left := max(grid[0][0], grid[n-1][n-1])
	right := n*n - 1
	ans := left + sort.Search(right-left, func(mx int) bool {
		mx += left
		var dfs func(int, int) bool
		dfs = func(i, j int) bool {
			if i == n-1 && j == n-1 {
				return true
			}

			vis[i][j] = mx
			for _, dir := range dirsSwimInWater {
				x, y := i+dir.x, j+dir.y
				if 0 <= x && x < n && 0 <= y && y < n && grid[x][y] <= mx && vis[x][y] != mx && dfs(x, y) {
					return true
				}
			}
			return false
		}
		return dfs(0, 0)
	})
	return ans
}
