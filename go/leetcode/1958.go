package leetcode

func checkMove(board [][]byte, rMove int, cMove int, color byte) bool {
	dirs := []struct{ x, y int }{{1, 0}, {1, 1}, {0, 1}, {-1, 1}, {-1, 0}, {-1, -1}, {0, -1}, {1, -1}}
	m, n := len(board), len(board[0])
	for _, dir := range dirs {
		x, y := rMove+dir.x, cMove+dir.y
		if x < 0 || x >= m || y < 0 || y >= n || board[x][y] != color^'B'^'W' {
			continue
		}
		for {
			x += dir.x
			y += dir.y
			if x < 0 || x >= m || y < 0 || y >= n || board[x][y] == '.' {
				break
			}
			if board[x][y] == color {
				return true
			}
		}
	}

	return false
}
