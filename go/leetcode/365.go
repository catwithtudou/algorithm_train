package leetcode

func canMeasureWater(x int, y int, z int) bool {
	type pair struct{ x, y int }
	visited := make(map[pair]bool)
	var dfs func(int, int) bool
	dfs = func(remainX int, remainY int) bool {
		st := pair{remainX, remainY}
		if visited[st] {
			return false
		}
		visited[st] = true
		if remainX == z || remainY == z || remainX+remainY == z {
			return true
		}
		return dfs(x, remainY) || dfs(remainX, y) || dfs(0, remainY) || dfs(remainX, 0) ||
			dfs(remainX-min(remainX, y-remainY), remainY+min(remainX, y-remainY)) ||
			dfs(remainX+min(remainY, x-remainX), remainY-min(remainY, x-remainX))
	}

	return dfs(0, 0)
}

func canMeasureWaterV1(x int, y int, z int) bool {
	if x+y < z {
		return false
	}
	if x == 0 || y == 0 {
		return z == 0 || x+y == z
	}
	return z%gcd(x, y) == 0
}

func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}
