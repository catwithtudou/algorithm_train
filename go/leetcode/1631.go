package leetcode

import "sort"

// 二分查找

type p struct{ x, y int }

var dirct = []p{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func minimumEffortPath(heights [][]int) int {
	n, m := len(heights), len(heights[0])
	return sort.Search(1e6, func(maxHeightDiff int) bool {
		vis := make([][]bool, n)
		for i := range vis {
			vis[i] = make([]bool, m)
		}
		vis[0][0] = true

		queue := []p{{}}
		for len(queue) > 0 {
			cur := queue[0]
			queue = queue[1:]
			if cur.x == n-1 && cur.y == m-1 {
				return true
			}

			for _, d := range dirct {
				x, y := cur.x+d.x, cur.y+d.y
				if x >= 0 && y >= 0 && x < n && y < m && !vis[x][y] {
					if abs(heights[x][y]-heights[cur.x][cur.y]) <= maxHeightDiff {
						vis[x][y] = true
						queue = append(queue, p{x, y})
					}

				}

			}

		}
		return false
	})
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

// 并查集

type UnionFind struct {
	parent, size []int
}

func NewUnionFind(n int) *UnionFind {
	parent := make([]int, n)
	size := make([]int, n)
	for i := range parent {
		parent[i] = i
		size[i] = 1
	}
	return &UnionFind{parent: parent, size: size}
}

func (uf *UnionFind) Find(x int) int {
	if uf.parent[x] != x {
		uf.parent[x] = uf.Find(uf.parent[x])
	}
	return uf.parent[x]
}

func (uf *UnionFind) Union(x, y int) {
	fx, fy := uf.Find(x), uf.Find(y)
	if fx == fy {
		return
	}

	if uf.size[fx] < uf.size[fy] {
		fx, fy = fy, fx
	}
	uf.size[fx] += uf.size[fy]
	uf.parent[fy] = fx
}

func (uf *UnionFind) InSameSet(x, y int) bool {
	return uf.Find(x) == uf.Find(y)
}

type edge struct {
	v, w, diff int
}

func minimumEffortPathByUnionFind(heights [][]int) int {
	n, m := len(heights), len(heights[0])
	var edges []edge
	for i, row := range heights {
		for j, h := range row {
			id := i*m + j
			if i > 0 {
				edges = append(edges, edge{id - m, id, abs(h - heights[i-1][j])})
			}
			if j > 0 {
				edges = append(edges, edge{id - 1, id, abs(h - heights[i][j-1])})
			}
		}
	}
	sort.Slice(edges, func(i, j int) bool {
		return edges[i].diff < edges[j].diff
	})

	uf := NewUnionFind(n * m)
	for _, e := range edges {
		uf.Union(e.v, e.w)
		if uf.InSameSet(0, n*m-1) {
			return e.diff
		}
	}
	return 0
}
