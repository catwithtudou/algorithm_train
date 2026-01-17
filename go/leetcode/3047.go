package leetcode

func largestSquareArea(bottomLeft [][]int, topRight [][]int) int64 {
	maxSide := 0

	for i, b1 := range bottomLeft {
		t1 := topRight[i]
		if t1[0]-b1[0] <= maxSide || t1[1]-b1[1] <= maxSide {
			continue
		}
		for j, b2 := range bottomLeft[:i] {
			t2 := topRight[j]
			width := min(t1[0], t2[0]) - max(b1[0], b2[0])
			height := min(t1[1], t2[1]) - max(b1[1], b2[1])
			maxSide = max(maxSide, min(width, height))
		}
	}

	return int64(maxSide) * int64(maxSide)
}
