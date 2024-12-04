package leetcode

type move struct {
	x0, y0 int
	dx, dy int
	step   int
}

func generateMoves(x0, y0 int, dirs []struct{ x, y int }) []move {
	const size = 8
	moves := []move{{x0, y0, 0, 0, 0}}
	for _, d := range dirs {
		x, y := x0+d.x, y0+d.y
		for step := 1; 0 < x && x <= size && 0 < y && y <= size; step++ {
			moves = append(moves, move{x0, y0, d.x, d.y, step})
			x += d.x
			y += d.y
		}
	}
	return moves
}

func isValid(m1, m2 move) bool {
	x1, y1 := m1.x0, m1.y0
	x2, y2 := m2.x0, m2.y0
	for i := range max(m1.step, m2.step) {
		if i < m1.step {
			x1 += m1.dx
			y1 += m1.dy
		}
		if i < m2.step {
			x2 += m2.dx
			y2 += m2.dy
		}
		if x1 == x2 && y1 == y2 {
			return false
		}
	}
	return true
}

var moveDirs = []struct{ x, y int }{{-1, 0}, {1, 0}, {0, -1}, {0, 1}, {-1, -1}, {-1, 1}, {1, 1}, {1, -1}}
var pieceDirs = map[byte][]struct{ x, y int }{
	'r': moveDirs[:4],
	'b': moveDirs[4:],
	'q': moveDirs,
}

func countCombinations(pieces []string, positions [][]int) int {
	n := len(pieces)
	allMoves := make([][]move, n)
	for i, pos := range positions {
		allMoves[i] = generateMoves(pos[0], pos[1], pieceDirs[pieces[i][0]])
	}

	path := make([]move, n)
	ans := 0
	var dfs func(int)
	dfs = func(i int) {
		if i == n {
			ans++
			return
		}
	outer:
		for _, move1 := range allMoves[i] {
			for _, move2 := range path[:i] {
				if !isValid(move1, move2) {
					continue outer
				}
			}
			path[i] = move1
			dfs(i + 1)
		}
	}
	dfs(0)
	return ans
}
