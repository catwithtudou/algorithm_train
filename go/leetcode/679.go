package leetcode

import (
	"math"
	"slices"
)

func judgePoint24(cards []int) bool {
	a := make([]float64, len(cards))
	for i, x := range cards {
		a[i] = float64(x)
	}

	return pointDfs(a)
}

func pointDfs(cards []float64) bool {

	const eps = 1e-6

	n := len(cards)

	if n == 1 {
		return math.Abs(cards[0]-24) < eps
	}

	for i, x := range cards {
		for j := i + 1; j < n; j++ {
			y := cards[j]

			candidates := []float64{x + y, x - y, y - x, x * y}
			if math.Abs(y) > eps {
				candidates = append(candidates, x/y)
			}

			if math.Abs(x) > eps {
				candidates = append(candidates, y/x)
			}

			newCards := append(slices.Clone(cards[:j]), cards[j+1:]...)

			for _, res := range candidates {
				newCards[i] = res
				if pointDfs(newCards) {
					return true
				}
			}

		}
	}

	return false
}
