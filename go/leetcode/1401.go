package leetcode

func checkOverlap(radius int, xCenter int, yCenter int, x1 int, y1 int, x2 int, y2 int) bool {
	x := max(x1, min(x2, xCenter))
	y := max(y1, min(y2, yCenter))

	return (x-xCenter)*(x-xCenter)+(y-yCenter)*(y-yCenter) <= radius*radius
}
