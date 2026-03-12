package leetcode

import "math"

type unionFindSp struct {
	fa []int // 代表元
	cc int   // 连通块个数
}

func newUnionFindSp(n int) unionFindSp {
	fa := make([]int, n)
	for i := range fa {
		fa[i] = i
	}
	return unionFindSp{fa, n}
}

func (u unionFindSp) find(x int) int {
	if u.fa[x] != x {
		u.fa[x] = u.find(u.fa[x])
	}
	return u.fa[x]
}

func (u *unionFindSp) merge(from, to int) bool {
	x, y := u.find(from), u.find(to)
	if x == y {
		return false
	}
	u.fa[x] = y
	u.cc--
	return true
}

func maxStability(n int, edges [][]int, k int) int {
	mustUf := newUnionFindSp(n) // 必选边
	allUf := newUnionFindSp(n)  // 所有边
	minS, maxS := math.MaxInt, 0
	for _, e := range edges {
		x, y, s, must := e[0], e[1], e[2], e[3]

		if must > 0 && !mustUf.merge(x, y) {
			return -1
		}

		allUf.merge(x, y)
		minS = min(minS, s)
		maxS = max(maxS, s)
	}

	if allUf.cc > 1 { // 图不连通
		return -1
	}

	check := func(low int) bool {
		u := newUnionFindSp(n)
		for _, e := range edges {
			x, y, s, must := e[0], e[1], e[2], e[3]
			if must > 0 && s < low { // 必选边的边权太小
				return false
			}
			if must > 0 || s >= low {
				u.merge(x, y)
			}
		}

		leftK := k
		for _, e := range edges {
			if leftK == 0 || u.cc == 1 {
				break
			}
			x, y, s, must := e[0], e[1], e[2], e[3]
			if must == 0 && s < low && s*2 >= low && u.merge(x, y) {
				leftK--
			}
		}
		return u.cc == 1
	}

	left, right := minS, maxS*2+1
	for left+1 < right {
		mid := left + (right-left)/2
		if check(mid) {
			left = mid
		} else {
			right = mid
		}
	}

	return left
}
