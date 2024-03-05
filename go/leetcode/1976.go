package leetcode

import "math"

func countPaths1976(n int, roads [][]int) int {
	g := make([][]int, n)
	for i := range g {
		g[i] = make([]int, n)
		for j := range g[i] {
			g[i][j] = math.MaxInt / 2
		}
	}

	for _, r := range roads {
		x, y, c := r[0], r[1], r[2]
		g[x][y] = c
		g[y][x] = c
	}

	dis := make([]int, n)
	for i := 1; i < n; i++ {
		dis[i] = math.MaxInt / 2
	}

	done := make([]bool, n)
	f := make([]int, n)
	f[0] = 1
	for {
		x := -1

		for i, ok := range done {
			if !ok && (x < 0 || dis[i] < dis[x]) {
				x = i
			}
		}

		if x == n-1 {
			return f[x]
		}

		done[x] = true
		for y, c := range g[x] {
			newCost := dis[x] + c
			if newCost < dis[y] {
				dis[y] = newCost
				f[y] = f[x]
			} else if newCost == dis[y] {
				f[y] = (f[x] + f[y]) % 1_000_000_007
			}

		}

	}

}
