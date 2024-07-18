package leetcode

import "container/heap"

func minimumTime3112(n int, edges [][]int, disappear []int) []int {
	type edge struct{ to, wt int }
	g := make([][]edge, n)
	for _, e := range edges {
		x, y, wt := e[0], e[1], e[2]
		g[x] = append(g[x], edge{y, wt})
		g[y] = append(g[y], edge{x, wt})
	}

	dis := make([]int, n)
	for i := range dis {
		dis[i] = -1
	}
	dis[0] = 0
	h := hpDis{{}}
	for len(h) > 0 {
		p := heap.Pop(&h).(pairDis)
		dx, x := p.dis, p.x
		if dx > dis[x] {
			continue
		}
		for _, e := range g[x] {
			y := e.to
			newDis := dx + e.wt
			if newDis < disappear[y] && (dis[y] < 0 || newDis < dis[y]) {
				dis[y] = newDis
				heap.Push(&h, pairDis{newDis, y})
			}

		}

	}

	return dis
}

type pairDis struct{ dis, x int }
type hpDis []pairDis

func (h hpDis) Len() int           { return len(h) }
func (h hpDis) Less(i, j int) bool { return h[i].dis < h[j].dis }
func (h hpDis) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *hpDis) Push(v any)        { *h = append(*h, v.(pairDis)) }
func (h *hpDis) Pop() (v any)      { a := *h; *h, v = a[:len(a)-1], a[len(a)-1]; return }
