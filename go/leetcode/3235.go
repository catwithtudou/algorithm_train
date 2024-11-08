package leetcode

func canReachCorner(xCorner int, yCorner int, circles [][]int) bool {
	vis := make([]bool, len(circles))
	var dfs func(int) bool
	dfs = func(i int) bool {
		x1, y1, r1 := circles[i][0], circles[i][1], circles[i][2]
		// 圆 i 是否与矩形右边界/下边界相交相切
		if y1 <= yCorner && abs(x1-xCorner) <= r1 || x1 <= xCorner && y1 <= r1 || x1 > xCorner && inCircle(x1, y1, r1, xCorner, 0) {
			return true
		}
		vis[i] = true
		for j, c := range circles {
			x2, y2, r2 := c[0], c[1], c[2]
			// 在两圆相交相切的前提下，点 A 是否严格在矩形内
			if !vis[j] && (x1-x2)*(x1-x2)+(y1-y2)*(y1-y2) <= (r1+r2)*(r1+r2) &&
				x1*r2+x2*r1 < (r1+r2)*xCorner &&
				y1*r2+y2*r1 < (r1+r2)*yCorner &&
				dfs(j) {
				return true
			}
		}
		return false

	}

	for i, c := range circles {
		x, y, r := c[0], c[1], c[2]
		if inCircle(x, y, r, 0, 0) || inCircle(x, y, r, xCorner, yCorner) ||
			!vis[i] && (x <= xCorner && abs(y-yCorner) <= r || y <= yCorner && x <= r || y > yCorner && inCircle(x, y, r, 0, yCorner)) && dfs(i) {
			return false
		}
	}
	return true
}

func inCircle(ox, oy, r, x, y int) bool {
	return (ox-x)*(ox-x)+(oy-y)*(oy-y) <= r*r
}
