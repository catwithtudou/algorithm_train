package leetcode

import (
	"container/heap"
	"math"
)

func shortestPath(n int, edges [][]int, labels string, k int) int {
	type edge struct{ to, w int }
	g := make([][]edge, n)
	for _, e := range edges {
		x, y, w := e[0], e[1], e[2]
		g[x] = append(g[x], edge{y, w})
	}

	dis := make([][]int, n)
	for i := range dis {
		dis[i] = make([]int, k+1)
		for j := range dis[i] {
			dis[i][j] = math.MaxInt
		}
	}

	h := treeHp{{0, 0, 1}}

	for len(h) > 0 {
		top := heap.Pop(&h).(treeTuple)
		d, x, cnt := top.dis, top.x, top.cnt
		if x == n-1 {
			return d
		}
		if d > dis[x][cnt] {
			continue
		}

		for _, e := range g[x] {
			y := e.to
			newCnt := 1
			if labels[y] == labels[x] {
				newCnt = cnt + 1
			}
			newDis := d + e.w
			if newCnt <= k && newDis < dis[y][newCnt] {
				dis[y][newCnt] = newDis
				heap.Push(&h, treeTuple{newDis, y, newCnt})
			}
		}
	}

	return -1
}

type treeTuple struct {
	dis, x, cnt int
}
type treeHp []treeTuple

func (h treeHp) Len() int           { return len(h) }
func (h treeHp) Less(i, j int) bool { return h[i].dis < h[j].dis }
func (h treeHp) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *treeHp) Push(v any)        { *h = append(*h, v.(treeTuple)) }
func (h *treeHp) Pop() (v any)      { a := *h; *h, v = a[:len(a)-1], a[len(a)-1]; return }
