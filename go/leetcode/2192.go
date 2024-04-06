package leetcode

import "sort"

func getAncestors(n int, edges [][]int) [][]int {
	anc := make([]map[int]bool, n) // 存储每个节点祖先的辅助数组
	for i := 0; i < n; i++ {
		anc[i] = map[int]bool{}
	}
	e := make([][]int, n)   // 邻接表
	indeg := make([]int, n) // 入度表
	// 预处理
	for _, edge := range edges {
		e[edge[0]] = append(e[edge[0]], edge[1])
		indeg[edge[1]]++
	}
	// 广度优先搜索求解拓扑排序
	var q []int
	for i := 0; i < n; i++ {
		if indeg[i] == 0 {
			q = append(q, i)
		}
	}
	for len(q) > 0 {
		u := q[0]
		q = q[1:]
		for _, v := range e[u] {
			// 更新子节点的祖先哈希表
			anc[v][u] = true
			for i := range anc[u] {
				anc[v][i] = true
			}
			indeg[v]--
			if indeg[v] == 0 {
				q = append(q, v)
			}
		}
	}
	// 转化为答案数组
	res := make([][]int, n)
	for i := 0; i < n; i++ {
		for j := range anc[i] {
			res[i] = append(res[i], j)
		}
		sort.Ints(res[i])
	}
	return res
}
