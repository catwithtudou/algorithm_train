package leetcode

import (
	"container/heap"
	"math"
)

func minimumVisitedCells(grid [][]int) int {
	colHeaps := make([]hp, len(grid[0]))
	rowH := hp{}
	f := 1
	for i, row := range grid {
		rowH = rowH[:0]
		for j, g := range row {
			for len(rowH) > 0 && rowH[0].idx < j {
				heap.Pop(&rowH)
			}
			colH := colHeaps[j]
			for len(colH) > 0 && colH[0].idx < i {
				heap.Pop(&colH)
			}

			if i > 0 || j > 0 {
				f = math.MaxInt
			}

			if len(rowH) > 0 {
				f = rowH[0].f + 1
			}
			if len(colH) > 0 {
				f = min(f, colH[0].f+1)
			}
			if g > 0 && f < math.MaxInt {
				heap.Push(&rowH, pairNum{f, g + j})
				heap.Push(&colH, pairNum{f, g + i})
			}
			colHeaps[j] = colH
		}
	}
	if f < math.MaxInt {
		return f
	}
	return -1
}

type pairNum struct{ f, idx int }
type hp []pairNum

func (h hp) Len() int           { return len(h) }
func (h hp) Less(i, j int) bool { return h[i].f < h[j].f }
func (h hp) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v any)        { *h = append(*h, v.(pairNum)) }
func (h *hp) Pop() any          { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }
