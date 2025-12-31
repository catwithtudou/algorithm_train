package leetcode

func latestDayToCross(m int, n int, cells [][]int) int {
	dirs := []struct{ x, y int }{{-1, -1}, {-1, 0}, {-1, 1}, {0, 1}, {1, 1}, {1, 0}, {1, -1}, {0, -1}}

	state := make([][]int8, m)
	for i := range state {
		state[i] = make([]int8, n)
	}

	canReachFromLeft := func(r, c int) bool {
		if c == 0 {
			return true
		}

		for _, d := range dirs {
			x, y := r+d.x, c+d.y
			if 0 <= x && x < m && 0 <= y && y < n && state[x][y] == 2 {
				return true
			}
		}

		return false
	}

	var dfs func(int, int) bool

	dfs = func(r, c int) bool {
		if c == n-1 {
			return true
		}

		state[r][c] = 2
		for _, d := range dirs {
			x, y := r+d.x, c+d.y
			if 0 <= x && x < m && 0 <= y && y < n && state[x][y] == 1 && dfs(x, y) {
				return true
			}
		}
		return false
	}

	for day, cell := range cells {
		r, c := cell[0]-1, cell[1]-1
		state[r][c] = 1
		if canReachFromLeft(r, c) && dfs(r, c) {
			return day
		}
	}

	return -1
}
