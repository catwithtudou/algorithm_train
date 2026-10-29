package leetcode

import (
	"strconv"
	"strings"
)

func palindromePath(n int, edges [][]int, s string, queries []string) []bool {
	g := make([][]int, n)
	for _, e := range edges {
		a, b := e[0], e[1]
		g[a] = append(g[a], b)
		g[b] = append(g[b], a)
	}

	mask := make([]int, n) // 每个节点字符的位掩码
	for i := range mask {
		mask[i] = 1 << (s[i] - 'a')
	}

	const LOG = 17 // 2^17 > 5e4
	var up [LOG][]int
	for k := range up {
		up[k] = make([]int, n)
	}
	depth := make([]int, n)
	tin := make([]int, n)  // dfs 序，从 1 开始
	tout := make([]int, n) // 子树里最大的 dfs 序
	px := make([]int, n)   // 初始的根到 u 前缀异或

	timer := 0
	var dfs func(u, p int)
	dfs = func(u, p int) {
		timer++
		tin[u] = timer
		up[0][u] = p
		for _, v := range g[u] {
			if v != p {
				depth[v] = depth[u] + 1
				px[v] = px[u] ^ mask[v]
				dfs(v, u)
			}
		}
		tout[u] = timer
	}
	px[0] = mask[0]
	dfs(0, 0)

	for k := 1; k < LOG; k++ {
		for u := 0; u < n; u++ {
			up[k][u] = up[k-1][up[k-1][u]]
		}
	}
	lca := func(u, v int) int {
		if depth[u] < depth[v] {
			u, v = v, u
		}
		for k := LOG - 1; k >= 0; k-- {
			if depth[u]-1<<k >= depth[v] {
				u = up[k][u]
			}
		}
		if u == v {
			return u
		}
		for k := LOG - 1; k >= 0; k-- {
			if up[k][u] != up[k][v] {
				u, v = up[k][u], up[k][v]
			}
		}
		return up[0][u]
	}

	// 树状数组：区间异或、单点查询（异或差分）
	bit := make([]int, n+1)
	add := func(i, d int) {
		for ; i <= n; i += i & -i {
			bit[i] ^= d
		}
	}
	pre := func(i int) int {
		r := 0
		for ; i > 0; i -= i & -i {
			r ^= bit[i]
		}
		return r
	}
	cur := func(u int) int { return px[u] ^ pre(tin[u]) } // 当前的根到 u 前缀异或

	ans := []bool{}
	for _, q := range queries {
		f := strings.Fields(q)
		a, _ := strconv.Atoi(f[1])
		if f[0] == "update" {
			nm := 1 << (f[2][0] - 'a')
			if d := mask[a] ^ nm; d != 0 {
				add(tin[a], d) // 子树区间 [tin, tout] 整体异或 d
				add(tout[a]+1, d)
				mask[a] = nm
			}
		} else {
			b, _ := strconv.Atoi(f[2])
			x := cur(a) ^ cur(b) ^ mask[lca(a, b)]
			ans = append(ans, x&(x-1) == 0) // 最多一位是 1
		}
	}
	return ans
}
