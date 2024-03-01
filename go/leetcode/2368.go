package leetcode

func reachableNodes(n int, edges [][]int, restricted []int) int {
	g := make([][]int, n)
	for _, v := range edges {
		a, b := v[0], v[1]
		g[a] = append(g[a], b)
		g[b] = append(g[b], a)
	}

	isRestricted := make([]int, n)
	for _, v := range restricted {
		isRestricted[v] = 1
	}

	seen := make([]int, n)
	count := 0
	s := []int{0}
	for len(s) > 0 {
		cur := s[len(s)-1]
		s = s[:len(s)-1]
		if seen[cur] == 1 {
			continue
		}

		seen[cur] = 1

		if isRestricted[cur] == 1 {
			continue
		}

		count++

		for _, v := range g[cur] {
			s = append(s, v)
		}
	}

	return count
}

type NormalUnionFind struct {
	f    []int
	rank []int
}

func NewNormalUnionFind(n int) *NormalUnionFind {
	uf := &NormalUnionFind{
		f:    make([]int, n),
		rank: make([]int, n),
	}
	for i := 0; i < n; i++ {
		uf.f[i] = i
	}
	return uf
}

func (uf *NormalUnionFind) merge(x, y int) {
	rx := uf.find(x)
	ry := uf.find(y)
	if rx != ry {
		if uf.rank[rx] > uf.rank[ry] {
			uf.f[ry] = rx
		} else if uf.rank[rx] < uf.rank[ry] {
			uf.f[rx] = ry
		} else {
			uf.f[ry] = rx
			uf.rank[rx]++
		}
	}
}

func (uf *NormalUnionFind) find(x int) int {
	if x != uf.f[x] {
		uf.f[x] = uf.find(uf.f[x])
	}
	return uf.f[x]
}

func (uf *NormalUnionFind) count() int {
	cnt := 0
	rt := uf.find(0)
	for i := 0; i < len(uf.f); i++ {
		if rt == uf.find(i) {
			cnt++
		}
	}
	return cnt
}

func reachableNodesUnionFind(n int, edges [][]int, restricted []int) int {
	isRestricted := make([]int, n)
	for _, x := range restricted {
		isRestricted[x] = 1
	}

	uf := NewNormalUnionFind(n)
	for _, v := range edges {
		if isRestricted[v[0]] == 1 || isRestricted[v[1]] == 1 {
			continue
		}
		uf.merge(v[0], v[1])
	}
	return uf.count()
}
