package leetcode

func numRookCaptures(board [][]byte) int {
	var x0, y0 int
	for i, x := range board {
		for j, y := range x {
			if y == 'R' {
				x0, y0 = i, j
				break
			}
		}
	}
	dirs := []struct{ x, y int }{{1, 0}, {0, 1}, {-1, 0}, {0, -1}}
	size := len(board)
	ans := 0
	for _, dir := range dirs {
		x, y := x0+dir.x, y0+dir.y
		for x >= 0 && x < size && y >= 0 && y < size && board[x][y] == '.' {
			x += dir.x
			y += dir.y
		}
		if x >= 0 && x < size && y >= 0 && y < size && board[x][y] == 'p' {
			ans++
		}
	}
	return ans
}
