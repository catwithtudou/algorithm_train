package leetcode

func minMoves3568(classroom []string, energy int) (ans int) {

	m, n := len(classroom), len(classroom[0])

	idx := make([][]int, m)

	for i := range idx {
		idx[i] = make([]int, n)
	}

	var cntL, sx, sy int

	for i, row := range classroom {
		for j, b := range row {
			if b == 'L' {
				idx[i][j] = 1 << cntL
				cntL++
			} else if b == 'S' {
				sx, sy = i, j
			}
		}
	}

	dirs := []struct{ x, y int }{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

	u := 1 << cntL

	vis := make([][][][]bool, m)
	for i := range vis {
		vis[i] = make([][][]bool, n)
		for j := range vis[i] {
			vis[i][j] = make([][]bool, energy+1)
			for k := range vis[i][j] {
				vis[i][j][k] = make([]bool, u)
			}
		}
	}

	vis[sx][sy][energy][0] = true

	type tuple struct{ x, y, e, mask int }
	q := []tuple{{sx, sy, energy, 0}}

	for ; len(q) > 0; ans++ {
		tmp := q
		q = nil
		for _, p := range tmp {
			if p.mask == u-1 {
				return
			}

			if p.e == 0 {
				continue
			}

			for _, d := range dirs {
				x, y := p.x+d.x, p.y+d.y
				if 0 <= x && x < m && 0 <= y && y < n && classroom[x][y] != 'X' {
					newE := p.e - 1
					if classroom[x][y] == 'R' {
						newE = energy
					}
					newMask := p.mask | idx[x][y]
					if !vis[x][y][newE][newMask] {
						vis[x][y][newE][newMask] = true
						q = append(q, tuple{x, y, newE, newMask})
					}
				}
			}
		}
	}
	return -1
}
