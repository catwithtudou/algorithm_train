package leetcode

import "math/bits"

func separateSquares(squares [][]int) float64 {
	totalArea := 0
	maxY := 0
	for _, sq := range squares {
		l := sq[2]
		totalArea += l * l
		maxY = max(maxY, sq[1]+l)
	}

	check := func(y float64) bool {
		area := 0.
		for _, sq := range squares {
			yi := float64(sq[1])
			if yi < y {
				l := float64(sq[2])
				area += l * minFloat64(y-yi, l)
			}
		}
		return area >= float64(totalArea)/2
	}

	left, right := 0., float64(maxY)
	for range bits.Len(uint(maxY * 1e5)) {
		mid := (right + left) / 2
		if check(mid) {
			right = mid
		} else {
			left = mid
		}
	}

	return (left + right) / 2
}
