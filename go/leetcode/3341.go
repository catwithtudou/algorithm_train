package leetcode

import (
	"container/heap"
	"math"
)

func minTimeToReach(moveTime [][]int) int {
	n, m := len(moveTime), len(moveTime[0])
	dis := make([][]int, n)
	for i := range dis {
		dis[i] = make([]int, m)
		for j := range dis[i] {
			dis[i][j] = math.MaxInt
		}
	}
	dis[0][0] = 0

	dirs := []struct{ x, y int }{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

	h := hpDirs{{}}

	for {
		top := heap.Pop(&h).(tupleDirs)
		i, j := top.x, top.y
		if i == n-1 && j == m-1 {
			return top.dis
		}

		if top.dis > dis[i][j] {
			continue
		}

		for _, d := range dirs {
			x, y := i+d.x, j+d.y
			if x >= 0 && x < n && y >= 0 && y < m {
				newDis := max(top.dis, moveTime[x][y]) + 1
				if newDis < dis[x][y] {
					dis[x][y] = newDis
					heap.Push(&h, tupleDirs{x, y, newDis})
				}
			}
		}
	}

}

type tupleDirs struct{ x, y, dis int }
type hpDirs []tupleDirs

func (h hpDirs) Len() int           { return len(h) }
func (h hpDirs) Less(i, j int) bool { return h[i].dis < h[j].dis }
func (h hpDirs) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *hpDirs) Push(v any)        { *h = append(*h, v.(tupleDirs)) }
func (h *hpDirs) Pop() (v any)      { a := *h; *h, v = a[:len(a)-1], a[len(a)-1]; return }
