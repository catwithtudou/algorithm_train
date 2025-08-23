package leetcode

import "math"

func minimumAreaII(grid [][]int, l, r int) int {
	left, right := r, 0
	top, bottom := len(grid), 0
	for i, row := range grid {
		for j, x := range row[l:r] {
			if x == 1 {
				left = min(left, j)
				right = max(right, j)
				top = min(top, i)
				bottom = i
			}
		}
	}
	return (right - left + 1) * (bottom - top + 1)
}

func minimumSumII(grid [][]int) int {

	ans := math.MaxInt

	solve := func(a [][]int) {
		m, n := len(a), len(a[0])
		if m >= 3 {
			for i := 1; i < m; i++ {
				for j := i + 1; j < m; j++ {
					area := minimumAreaII(a[:i], 0, n)
					area += minimumAreaII(a[i:j], 0, n)
					area += minimumAreaII(a[j:], 0, n)
					ans = min(ans, area)
				}
			}
		}

		if m >= 2 && n >= 2 {
			for i := 1; i < m; i++ {
				for j := 1; j < n; j++ {
					area := minimumAreaII(a[:i], 0, n)
					area += minimumAreaII(a[i:], 0, j)
					area += minimumAreaII(a[i:], j, n)
					ans = min(ans, area)

					area = minimumAreaII(a[:i], 0, j)
					area += minimumAreaII(a[:i], j, n)
					area += minimumAreaII(a[i:], 0, n)
					ans = min(ans, area)
				}
			}
		}
	}

	solve(grid)
	solve(rotate(grid))

	return ans
}

func rotate(a [][]int) [][]int {
	m, n := len(a), len(a[0])
	b := make([][]int, n)
	for i := range b {
		b[i] = make([]int, m)
	}
	for i, row := range a {
		for j, x := range row {
			b[j][m-1-i] = x
		}
	}
	return b
}
