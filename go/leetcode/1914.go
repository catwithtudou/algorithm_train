package leetcode

var dirsGird = [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}

func rotateGrid1914(grid [][]int, k int) [][]int {
	m0, n0 := len(grid), len(grid[0])
	a := make([]int, 0, (m0+n0-2)*2)

	for i := range min(m0, n0) / 2 {
		m, n := m0-i*2, n0-i*2
		x, y := i, i
		a = a[:0]

		for _, dir := range dirsGird {
			for range n - 1 {
				a = append(a, grid[x][y])
				x += dir[0]
				y += dir[1]
			}
			m, n = n, m
		}

		shift := k % len(a)
		a = append(a[shift:], a[:shift]...)

		j := 0
		for _, dir := range dirsGird {
			for range n - 1 {
				grid[x][y] = a[j]
				j++
				x += dir[0]
				y += dir[1]
			}
			m, n = n, m
		}
	}

	return grid
}
