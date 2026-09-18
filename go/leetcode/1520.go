package leetcode

import (
	"sort"

	"golang.org/x/exp/slices"
)

func maxNumOfSubstrings(s string) (ans []string) {
	pos := [26][]int{}
	for i, b := range s {
		b -= 'a'
		pos[b] = append(pos[b], i)
	}

	g := [26][]int{}
	for i, p := range pos {
		if p == nil {
			continue
		}
		l, r := p[0], p[len(p)-1]
		for j, q := range pos {
			if j == i {
				continue
			}
			k := sort.SearchInts(q, l)
			if k < len(q) && q[k] <= r {
				g[i] = append(g[i], j)
			}
		}
	}

	vis := [26]bool{}
	var l, r int
	var dfs func(int)
	dfs = func(x int) {
		vis[x] = true
		p := pos[x]
		l = min(l, p[0])
		r = max(r, p[len(p)-1])
		for _, y := range g[x] {
			if !vis[y] {
				dfs(y)
			}
		}
	}

	type pair struct{ l, r int }
	intervals := []pair{}
	for i, p := range pos {
		if p == nil {
			continue
		}

		vis = [26]bool{}
		l, r = len(s), 0
		dfs(i)
		intervals = append(intervals, pair{l, r})
	}

	slices.SortFunc(intervals, func(a, b pair) int { return a.r - b.r })
	preR := -1
	for _, p := range intervals {
		if p.l > preR {
			ans = append(ans, s[p.l:p.r+1])
			preR = p.r
		}
	}
	return
}
