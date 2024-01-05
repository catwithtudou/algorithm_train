package leetcode

import "math/bits"

func maximumRows(matrix [][]int, numSelect int) int {
	m, n := len(matrix), len(matrix[0])
	ans := 0
	rows := make([]int, m)
	for i, row := range matrix {
		for j, x := range row {
			if x == 1 {
				rows[i] |= 1 << j
			}
		}
	}
	for mask := 1; mask < 1<<n; mask++ {
		if bits.OnesCount(uint(mask)) != numSelect {
			continue
		}
		t := 0
		for _, x := range rows {
			if (x & mask) == x {
				t++
			}
		}
		ans = max(ans, t)
	}

	return ans
}
