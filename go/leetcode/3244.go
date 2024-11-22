package leetcode

func shortestDistanceAfterQueriesII(n int, queries [][]int) []int {
	fa := make([]int, n-1)
	for i := range fa {
		fa[i] = i
	}

	find := func(x int) int {
		rt := x
		for fa[rt] != rt {
			rt = fa[rt]
		}
		for fa[x] != rt {
			fa[x], x = rt, fa[x]
		}
		return rt
	}

	ans := make([]int, len(queries))
	cnt := n - 1
	for i, q := range queries {
		l, r := q[0], q[1]-1
		rt := find(r)
		for i := find(l); i < r; i = find(i + 1) {
			fa[i] = rt
			cnt--
		}
		ans[i] = cnt
	}

	return ans
}
