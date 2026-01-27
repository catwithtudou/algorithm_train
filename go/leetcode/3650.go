package leetcode

import (
	"container/heap"
	"math"
)

func minCost3650(n int, edges [][]int) int {
	type edge struct{ to, wt int }
	g := make([][]edge, n)
	for _, e := range edges {
		x, y, wt := e[0], e[1], e[2]
		g[x] = append(g[x], edge{y, wt})
		g[y] = append(g[y], edge{x, wt * 2})
	}

	dis := make([]int, n)
	for i := range dis {
		dis[i] = math.MaxInt
	}
	dis[0] = 0
	h := &hpDis{{}}

	for h.Len() > 0 {
		p := heap.Pop(h).(pairDis)
		disX, x := p.dis, p.x
		if disX > dis[x] {
			continue
		}
		if x == n-1 {
			return disX
		}

		for _, e := range g[x] {
			y := e.to
			newDisY := disX + e.wt
			if newDisY < dis[y] {
				dis[y] = newDisY
				heap.Push(h, pairDis{newDisY, y})
			}
		}
	}

	return -1
}
