package leetcode

import "sort"

type pair struct{ x, y int }

var dirs = []pair{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func maximumMinutes(grid [][]int) int {
	m, n := len(grid), len(grid[0])

	ans := sort.Search(m*n+1, func(t int) bool {
		onFire := make([][]bool, m)
		for i := range onFire {
			onFire[i] = make([]bool, n)
		}

		f := []pair{}
		for i, row := range grid {
			for j, x := range row {
				if x == 1 {
					onFire[i][j] = true
					f = append(f, pair{i, j})
				}
			}
		}

		// 火焰的 BFS
		fireBfs := func() {
			tmp := f
			f = nil
			for _, p := range tmp {
				for _, d := range dirs {
					x, y := p.x+d.x, p.y+d.y
					if 0 <= x && x < m && 0 <= y && y < n {
						if !onFire[x][y] && grid[x][y] == 0 {
							onFire[x][y] = true
							f = append(f, pair{x, y})
						}
					}
				}
			}
		}

		for ; t > 0 && len(f) > 0; t-- {
			fireBfs()
		}
		if onFire[0][0] {
			return true
		}

		// 人的 BFS
		vis := make([][]bool, m)
		for i := range vis {
			vis[i] = make([]bool, n)
		}
		vis[0][0] = true
		q := []pair{{}}
		for len(q) > 0 {
			tmp := q
			q = nil
			for _, p := range tmp {
				if onFire[p.x][p.y] {
					continue
				}
				for _, d := range dirs {
					x, y := p.x+d.x, p.y+d.y
					if 0 <= x && x < m && 0 <= y && y < n {
						if !vis[x][y] && !onFire[x][y] && grid[x][y] == 0 {
							if x == m-1 && y == n-1 {
								return false
							}
							vis[x][y] = true
							q = append(q, pair{x, y})
						}
					}
				}
			}
			fireBfs()
		}
		return true
	}) - 1
	if ans < m*n {
		return ans
	}
	return 1_000_000_000
}
