package leetcode

import "math"

type pairXy struct{ x, y int }

func minimumMoves2850(grid [][]int) int {
	var from, to []pairXy
	for i, row := range grid {
		for j, col := range row {
			if col > 1 {
				for k := 1; k < col; k++ {
					from = append(from, pairXy{i, j})
				}
			} else if col == 0 {
				to = append(to, pairXy{i, j})
			}
		}
	}

	ans := math.MaxInt
	permute(from, 0, func() {
		total := 0
		for i, p := range from {
			total += abs(p.x-to[i].x) + abs(p.y-to[i].y)
		}
		ans = min(ans, total)
	})
	return ans
}

func permute(a []pairXy, i int, do func()) {
	if i == len(a) {
		do()
		return
	}
	permute(a, i+1, do)
	for j := i + 1; j < len(a); j++ {
		a[i], a[j] = a[j], a[i]
		permute(a, i+1, do)
		a[i], a[j] = a[j], a[i]
	}
}
