package leetcode

import "math"

type Graph [][]int

const inf = math.MaxInt / 2

func ConstructorGraph(n int, edges [][]int) Graph {
	g := make([][]int, n)
	for i := range g {
		g[i] = make([]int, n)
		for j := range g[i] {
			g[i][j] = inf
		}
	}
	for _, e := range edges {
		g[e[0]][e[1]] = e[2]
	}
	return g
}

func (g Graph) AddEdge(edge []int) {
	g[edge[0]][edge[1]] = edge[2]
}

func (g Graph) ShortestPath(node1 int, node2 int) int {
	n := len(g)
	dis := make([]int, n)
	for i := range dis {
		dis[i] = inf
	}
	dis[node1] = 0
	vis := make([]bool, n)
	for {
		x := -1
		for i, b := range vis {
			if !b && (x < 0 || dis[i] < dis[x]) {
				x = i
			}
		}
		if x < 0 || dis[x] == inf {
			return -1
		}
		if x == node2 {
			return dis[x]
		}
		vis[x] = true
		for y, w := range g[x] {
			dis[y] = min(dis[y], dis[x]+w)
		}
	}
}

/**
 * Your Graph object will be instantiated and called as such:
 * obj := Constructor(n, edges);
 * obj.AddEdge(edge);
 * param_2 := obj.ShortestPath(node1,node2);
 */
