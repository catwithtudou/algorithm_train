package leetcode

import (
	"container/heap"
	"sort"
)

func leftmostBuildingQueries(heights []int, queries [][]int) []int {
	ans := make([]int, len(queries))
	for i := range ans {
		ans[i] = -1
	}

	qs := make([][]pairQuery, len(heights))
	for i, q := range queries {
		a, b := q[0], q[1]
		if a > b {
			a, b = b, a
		}
		if a == b || heights[a] < heights[b] {
			ans[i] = b
		} else {
			qs[b] = append(qs[b], pairQuery{heights[a], i})
		}
	}

	h := queryHp{}
	for i, x := range heights {
		for h.Len() > 0 && h[0].h < x {
			ans[heap.Pop(&h).(pairQuery).i] = i
		}
		for _, p := range qs[i] {
			heap.Push(&h, p)
		}
	}
	return ans
}

type pairQuery struct {
	h, i int
}

type queryHp []pairQuery

func (q queryHp) Len() int           { return len(q) }
func (q queryHp) Less(i, j int) bool { return q[i].h < q[j].h }
func (q queryHp) Swap(i, j int)      { q[i], q[j] = q[j], q[i] }
func (q *queryHp) Push(v any)        { *q = append(*q, v.(pairQuery)) }
func (q *queryHp) Pop() any          { a := *q; v := a[len(a)-1]; *q = a[:len(a)-1]; return v }

func leftmostBuildingQueries2940(heights []int, queries [][]int) []int {
	ans := make([]int, len(queries))
	type pair struct {
		h, i int
	}
	qs := make([][]pair, len(heights))
	for i, q := range queries {
		a, b := q[0], q[1]
		if a > b {
			a, b = b, a
		}
		if a == b || heights[a] < heights[b] {
			ans[i] = b
		} else {
			qs[b] = append(qs[b], pair{heights[a], i})
		}
	}

	var st []int
	for i := len(heights) - 1; i >= 0; i-- {
		for _, q := range qs[i] {
			j := sort.Search(len(st), func(i int) bool {
				return heights[st[i]] <= q.h
			}) - 1
			if j >= 0 {
				ans[q.i] = st[j]
			} else {
				ans[q.i] = -1
			}
		}
		for len(st) > 0 && heights[i] >= heights[st[len(st)-1]] {
			st = st[:len(st)-1]
		}
		st = append(st, i)
	}

	return ans
}
