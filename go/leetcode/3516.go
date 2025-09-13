package leetcode

func findClosest(x int, y int, z int) int {
	if abs(x-z) < abs(y-z) {
		return 1
	} else if abs(x-z) > abs(y-z) {
		return 2
	}
	return 0
}
