package leetcode

func minTimeToVisitAllPoints(points [][]int) (ans int) {
	for i := 1; i < len(points); i++ {
		p := points[i-1]
		q := points[i]
		ans += max(abs(p[0]-q[0]), abs(p[1]-q[1]))
	}
	return
}
