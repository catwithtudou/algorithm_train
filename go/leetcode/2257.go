package leetcode

func countUnguarded(m int, n int, guards [][]int, walls [][]int) int {
	grid := make([][]int, m)
	for i := range grid {
		grid[i] = make([]int, n)
	}

	q := [][3]int{}
	dx := []int{1, 0, -1, 0}
	dy := []int{0, 1, 0, -1}
	for _, guard := range guards {
		grid[guard[0]][guard[1]] = -1
		for k := 0; k < 4; k++ {
			q = append(q, [3]int{guard[0], guard[1], k})
		}
	}
	for _, wall := range walls {
		grid[wall[0]][wall[1]] = -2
	}

	for len(q) > 0 {
		curr := q[0]
		q = q[1:]
		x, y, k := curr[0], curr[1], curr[2]
		nx := x + dx[k]
		ny := y + dy[k]
		if nx >= 0 && nx < m && ny >= 0 && ny < n && grid[nx][ny] >= 0 {
			if (grid[nx][ny] & (1 << k)) == 0 {
				grid[nx][ny] |= (1 << k)
				q = append(q, [3]int{nx, ny, k})
			}
		}
	}

	res := 0
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == 0 {
				res++
			}
		}
	}
	return res
}
