package leetcode

import (
	"math"

	"github.com/emirpasic/gods/v2/trees/redblacktree"
)

func minimumDistance(points [][]int) int {
	sx := redblacktree.New[int, int]()
	sy := redblacktree.New[int, int]()
	for _, p := range points {
		put(sx, p[0]-p[1])
		put(sy, p[1]+p[0])
	}
	res := math.MaxInt
	for _, p := range points {
		remove(sx, p[0]-p[1])
		remove(sy, p[1]+p[0])
		res = min(res, max(sx.Right().Key-sx.Left().Key, sy.Right().Key-sy.Left().Key))
		put(sx, p[0]-p[1])
		put(sy, p[1]+p[0])
	}
	return res
}
func put(t *redblacktree.Tree[int, int], v int) {
	c, _ := t.Get(v)
	t.Put(v, c+1)
}

func remove(t *redblacktree.Tree[int, int], v int) {
	c, _ := t.Get(v)
	if c == 1 {
		t.Remove(v)
	} else {
		t.Put(v, c-1)
	}
}
