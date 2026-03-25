package leetcode

func constructProductMatrix(grid [][]int) [][]int {
	const mod = 12345
	n, m := len(grid), len(grid[0])
	p := make([][]int, n)

	suf := 1
	for i := n - 1; i >= 0; i-- {
		p[i] = make([]int, m)
		for j := m - 1; j >= 0; j-- {
			p[i][j] = suf
			suf = suf * grid[i][j] % mod
		}
	}

	pre := 1
	for i, row := range grid {
		for j, v := range row {
			p[i][j] = p[i][j] * pre % mod
			pre = pre * v % mod
		}
	}

	return p
}
