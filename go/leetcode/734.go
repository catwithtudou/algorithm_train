package leetcode

import (
	"math"

	"golang.org/x/exp/slices"
)

func networkDelayTime(times [][]int, n int, k int) int {
	const inf = math.MaxInt / 2
	g := make([][]int, n)
	for i := range g {
		g[i] = make([]int, n)
		for j := range g[i] {
			g[i][j] = inf
		}
	}
	for _, t := range times {
		g[t[0]-1][t[1]-1] = t[2]
	}

	dis := make([]int, n)
	for i := range dis {
		dis[i] = inf
	}
	dis[k-1] = 0
	done := make([]bool, n)
	for {
		x := -1
		for i, ok := range done {
			if !ok && (x < 0 || dis[i] < dis[x]) {
				x = i
			}
		}
		if x < 0 {
			return slices.Max(dis)
		}
		if dis[x] == inf {
			return -1
		}
		done[x] = true
		for y, d := range g[x] {
			dis[y] = min(dis[y], dis[x]+d)
		}

	}
}
