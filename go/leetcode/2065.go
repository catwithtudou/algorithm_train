package leetcode

import (
	"container/heap"
	"math"
)

func maximalPathQuality(values []int, edges [][]int, maxTime int) int {
	n := len(values)
	type edge struct {
		to, time int
	}
	g := make([][]edge, n)
	for _, e := range edges {
		x, y, t := e[0], e[1], e[2]
		g[x] = append(g[x], edge{y, t})
		g[y] = append(g[y], edge{x, t})
	}

	dis := make([]int, n)
	for i := 1; i < n; i++ {
		dis[i] = math.MaxInt
	}
	h := edgeHp{{0, 0}}
	for len(h) > 0 {
		p := heap.Pop(&h).(edgePair)
		dx := p.dis
		x := p.x
		if dx > dis[x] {
			continue
		}
		for _, e := range g[x] {
			y := e.to
			newDis := dx + e.time
			if newDis < dis[y] {
				dis[y] = newDis
				heap.Push(&h, edgePair{newDis, y})
			}
		}
	}

	vis := make([]bool, n)
	vis[0] = true
	ans := 0
	var dfs func(int, int, int)
	dfs = func(x int, sumTime int, sumValue int) {
		if x == 0 {
			ans = max(ans, sumValue)
		}
		for _, e := range g[x] {
			y, t := e.to, e.time
			if sumTime+t+dis[y] > maxTime {
				continue
			}
			if vis[y] {
				dfs(y, sumTime+t, sumValue)
			} else {
				vis[y] = true
				dfs(y, sumTime+t, sumValue+values[y])
				vis[y] = false
			}
		}
	}
	dfs(0, 0, values[0])
	return ans
}

type edgePair struct{ dis, x int }
type edgeHp []edgePair

func (h edgeHp) Len() int           { return len(h) }
func (h edgeHp) Less(i, j int) bool { return h[i].dis < h[j].dis }
func (h edgeHp) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *edgeHp) Push(v any)        { *h = append(*h, v.(edgePair)) }
func (h *edgeHp) Pop() (v any)      { a := *h; *h, v = a[:len(a)-1], a[len(a)-1]; return }
