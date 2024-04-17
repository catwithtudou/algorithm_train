package leetcode

import "golang.org/x/exp/slices"

func minMalwareSpread(graph [][]int, initial []int) int {
	vis := make([]bool, len(graph))
	initialBool := make([]bool, len(graph))
	for _, v := range initial {
		initialBool[v] = true
	}

	var size, nodeId int
	var dfs func(x int)
	dfs = func(x int) {
		vis[x] = true
		size++
		if nodeId != -2 && initialBool[x] {
			if nodeId < 0 {
				nodeId = x
			} else {
				nodeId = -2
			}
		}

		for i, v := range graph[x] {
			if !vis[i] && v == 1 {
				dfs(i)
			}
		}
	}

	ans := -1
	maxSize := 0
	for _, v := range initial {
		if vis[v] {
			continue
		}
		nodeId = -1
		size = 0
		dfs(v)
		if nodeId >= 0 && (size > maxSize || (size == maxSize && nodeId < ans)) {
			ans = nodeId
			maxSize = size
		}
	}

	if ans < 0 {
		return slices.Min(initial)
	}

	return ans
}
