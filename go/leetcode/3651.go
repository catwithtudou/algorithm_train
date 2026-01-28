package leetcode

import (
	"math"

	"golang.org/x/exp/slices"
)

func minCost3651(grid [][]int, k int) int {
	n := len(grid[0])
	mx := 0
	for _, row := range grid {
		mx = max(mx, slices.Max(row))
	}

	sufMinF := make([]int, mx+2)
	for i := range sufMinF {
		sufMinF[i] = math.MaxInt
	}
	minF := make([]int, mx+1)
	f := make([]int, n+1)

	for range k + 1 {
		for i := range minF {
			minF[i] = math.MaxInt
		}

		for i := range f {
			f[i] = math.MaxInt / 2
		}
		f[1] = -grid[0][0]
		for _, row := range grid {
			for j, x := range row {
				f[j+1] = min(f[j]+x, min(f[j+1]+x, sufMinF[x]))
				minF[x] = min(minF[x], f[j+1])
			}
		}

		for i := mx; i >= 0; i-- {
			sufMinF[i] = min(sufMinF[i+1], minF[i])
		}
	}

	return f[n]
}
