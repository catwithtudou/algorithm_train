package leetcode

import "math/bits"

var pow2Info = [1e5]int{1}

func initAssignEdgeWeightsII() {
	for i := 1; i < len(pow2Info); i++ {
		pow2Info[i] = pow2Info[i-1] * 2 % mod
	}
}

func assignEdgeWeightsII(edges [][]int, queries [][]int) []int {
	n := len(edges) + 1
	g := make([][]int, n)
	for _, e := range edges {
		x, y := e[0]-1, e[1]-1
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}

	const mx = 17
	pa := make([][mx]int, n)
	dep := make([]int, n)
	var dfs func(int, int)
	dfs = func(x, p int) {
		pa[x][0] = p
		for _, y := range g[x] {
			if y != p {
				dep[y] = dep[x] + 1
				dfs(y, x)
			}
		}
	}
	dfs(0, -1)

	for i := range mx - 1 {
		for x := range pa {
			if p := pa[x][i]; p != -1 {
				pa[x][i+1] = pa[p][i]
			} else {
				pa[x][i+1] = -1
			}
		}
	}

	uptoDep := func(x, d int) int {
		for k := uint(dep[x] - d); k > 0; k &= k - 1 {
			x = pa[x][bits.TrailingZeros(k)]
		}
		return x
	}

	getLCA := func(x, y int) int {
		if dep[x] > dep[y] {
			x, y = y, x
		}
		y = uptoDep(y, dep[x])
		if x == y {
			return x
		}
		for i := mx - 1; i >= 0; i-- {
			if pv, pw := pa[x][i], pa[y][i]; pv != pw {
				x, y = pv, pw
			}
		}
		return pa[x][0]
	}

	getDis := func(x, y int) int { return dep[x] + dep[y] - 2*dep[getLCA(x, y)] }

	ans := make([]int, len(queries))
	for i, q := range queries {
		x, y := q[0], q[1]
		if x != y {
			ans[i] = pow2Info[getDis(x-1, y-1)-1]
		}
	}
	return ans
}
