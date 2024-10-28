package leetcode

func findRedundantDirectedConnection(edges [][]int) []int {
	n := len(edges)
	uf := newConUnionFind(n + 1)
	parent := make([]int, n+1)
	for i := range parent {
		parent[i] = i
	}

	var confictEdge, cycleEdge []int
	for _, edge := range edges {
		from, to := edge[0], edge[1]
		if parent[to] != to {
			confictEdge = edge
		} else {
			parent[to] = from
			if uf.find(to) == uf.find(from) {
				cycleEdge = edge
			} else {
				uf.union(from, to)
			}
		}
	}

	if confictEdge == nil {
		return cycleEdge
	}

	if cycleEdge != nil {
		return []int{parent[confictEdge[1]], confictEdge[1]}
	}

	return confictEdge
}

type conUnionFind struct {
	anc []int
}

func newConUnionFind(n int) conUnionFind {
	anc := make([]int, n)
	for i := range anc {
		anc[i] = i
	}
	return conUnionFind{anc}
}

func (uf conUnionFind) find(x int) int {
	if uf.anc[x] != x {
		uf.anc[x] = uf.find(uf.anc[x])
	}
	return uf.anc[x]
}

func (uf conUnionFind) union(from, to int) {
	uf.anc[uf.find(from)] = uf.find(to)
}
