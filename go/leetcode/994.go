package leetcode

func orangesRotting(grid [][]int) int {
	R, C := len(grid), len(grid[0])
	dr := []int{-1, 0, 1, 0}
	dc := []int{0, -1, 0, 1}
	queue := []int{}
	depth := make(map[int]int)

	for r := 0; r < R; r++ {
		for c := 0; c < C; c++ {
			if grid[r][c] == 2 {
				code := r*C + c
				queue = append(queue, code)
				depth[code] = 0
			}
		}
	}

	ans := 0
	for len(queue) > 0 {
		code := queue[0]
		queue = queue[1:]
		r, c := code/C, code%C
		for k := 0; k < 4; k++ {
			nr, nc := r+dr[k], c+dc[k]
			if nr >= 0 && nr < R && nc >= 0 && nc < C && grid[nr][nc] == 1 {
				grid[nr][nc] = 2
				ncode := nr*C + nc
				queue = append(queue, ncode)
				depth[ncode] = depth[code] + 1
				ans = depth[ncode]
			}
		}
	}

	for _, row := range grid {
		for _, v := range row {
			if v == 1 {
				return -1
			}
		}
	}
	return ans
}
