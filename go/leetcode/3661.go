package leetcode

import (
	"sort"

	"golang.org/x/exp/slices"
)

func maxWalls(robots []int, distance []int, walls []int) int {
	n := len(robots)

	type pair struct{ x, d int }

	a := make([]pair, n)
	for i, x := range robots {
		a[i] = pair{x, distance[i]}
	}

	slices.SortFunc(a, func(a pair, b pair) int {
		return a.x - b.x
	})
	slices.Sort(walls)

	memo := make([][2]int, n)
	for i := range memo {
		memo[i] = [2]int{-1, -1}
	}

	var dfs func(int, int) int

	dfs = func(i, j int) int {
		if i < 0 {
			return 0
		}

		c := &memo[i][j]
		if *c != -1 {
			return *c
		}

		leftX := a[i].x - a[i].d
		if i > 0 {
			leftX = max(leftX, a[i-1].x+1)
		}

		left := sort.SearchInts(walls, leftX)
		cur := sort.SearchInts(walls, a[i].x+1)

		res := dfs(i-1, 0) + cur - left

		rightX := a[i].x + a[i].d

		if i+1 < n {
			x2 := a[i+1].x
			if j == 0 {
				x2 -= a[i+1].d // 右边往左射击
			}
			rightX = min(rightX, x2-1)
		}

		right := sort.SearchInts(walls, rightX+1)
		cur = sort.SearchInts(walls, a[i].x)

		res = max(res, dfs(i-1, 1)+right-cur)

		*c = res
		return res
	}

	return dfs(n-1, 1)
}
