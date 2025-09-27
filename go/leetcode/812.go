package leetcode

func largestTriangleArea(points [][]int) float64 {
	n := len(points)
	ans := 0
	for i := range n - 2 {
		for j := i + 1; j < n-1; j++ {
			for k := j + 1; k < n; k++ {
				p1, p2, p3 := points[i], points[j], points[k]
				x1, y1 := p2[0]-p1[0], p2[1]-p1[1]
				x2, y2 := p3[0]-p1[0], p3[1]-p1[1]
				area := abs(x1*y2 - x2*y1)
				if area > ans {
					ans = area
				}
			}
		}
	}

	return float64(ans) / 2
}
