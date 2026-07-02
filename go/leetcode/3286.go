package leetcode

import "math"

func findSafeWalk(grid [][]int, health int) bool {

	m, n := len(grid), len(grid[0])
	dis := make([][]int, m)
	for i := range dis {
		dis[i] = make([]int, n)
		for j := range dis[i] {
			dis[i][j] = math.MaxInt
		}
	}

	dis[0][0] = grid[0][0]
	q := [2][]pair{{{}}}
	for len(q[0]) > 0 || len(q[1]) > 0 {
		var p pair
		if len(q[0]) > 0 {
			p, q[0] = q[0][len(q[0])-1], q[0][:len(q[0])-1]
		} else {
			p, q[1] = q[1][0], q[1][1:]
		}

		i, j := p.x, p.y
		for _, d := range dir4 {
			x, y := i+d.x, j+d.y
			if 0 <= x && x < m && 0 <= y && y < n {
				cost := grid[x][y]
				if dis[i][j]+cost < dis[x][y] {
					dis[x][y] = dis[i][j] + cost
					q[cost] = append(q[cost], pair{x, y})
				}
			}
		}

	}

	return dis[m-1][n-1] < health
}
