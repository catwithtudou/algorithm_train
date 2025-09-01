package leetcode

import "container/heap"

func solveSudoku(board [][]byte) {
	rowHas := [9][9]bool{}       // rowHas[i][x] 表示 i 行是否有数字 x
	colHas := [9][9]bool{}       // colHas[j][x] 表示 j 列是否有数字 x
	subBoxHas := [3][3][9]bool{} // subBoxHas[i'][j'][x] 表示 (i',j') 宫是否有数字 x
	emptyPos := [][2]int{}       // 空格子的位置

	for i := range 9 {
		for j := range 9 {
			if board[i][j] == '.' {
				emptyPos = append(emptyPos, [2]int{i, j})
			} else {
				num := board[i][j] - '1'
				rowHas[i][num] = true
				colHas[j][num] = true
				subBoxHas[i/3][j/3][num] = true
			}
		}
	}

	getCandidates := func(i, j int) int {
		candidates := 9
		for x := range 9 {
			if rowHas[i][x] || colHas[j][x] || subBoxHas[i/3][j/3][x] {
				candidates--
			}
		}
		return candidates
	}

	emptyHeap := &sudokuHp{}
	for _, pos := range emptyPos {
		i, j := pos[0], pos[1]
		heap.Push(emptyHeap, sudokuTuple{getCandidates(i, j), i, j})
	}
	heap.Init(emptyHeap)

	var dfs func() bool
	dfs = func() bool {
		if emptyHeap.Len() == 0 {
			return true
		}

		t := heap.Pop(emptyHeap).(sudokuTuple)
		i, j := t.i, t.j
		candidates := 0

		for x := range 9 {
			if rowHas[i][x] || colHas[j][x] || subBoxHas[i/3][j/3][x] {
				continue
			}

			board[i][j] = byte(x + '1')
			rowHas[i][x], colHas[j][x], subBoxHas[i/3][j/3][x] = true, true, true

			if dfs() {
				return true
			}

			rowHas[i][x], colHas[j][x], subBoxHas[i/3][j/3][x] = false, false, false

			candidates++
		}

		heap.Push(emptyHeap, sudokuTuple{candidates, i, j})

		return false
	}

	dfs()
}

type sudokuTuple struct{ candidates, i, j int }
type sudokuHp []sudokuTuple

func (h sudokuHp) Len() int           { return len(h) }
func (h sudokuHp) Less(i, j int) bool { return h[i].candidates < h[j].candidates }
func (h sudokuHp) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *sudokuHp) Push(v any)        { *h = append(*h, v.(sudokuTuple)) }
func (h *sudokuHp) Pop() any          { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }
