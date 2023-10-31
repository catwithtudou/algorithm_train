package leetcode

import "golang.org/x/exp/slices"

func smallestMissingValueSubtree(parents []int, nums []int) []int {
	n := len(parents)
	ans := make([]int, n)
	for i := range ans {
		ans[i] = 1
	}

	node := slices.Index(nums, 1)
	if node < 0 {
		return ans
	}

	tree := make([][]int, n)
	for i := 1; i < n; i++ {
		p := parents[i]
		tree[p] = append(tree[p], i)
	}

	visited := make(map[int]bool, n)
	var dfs func(int)
	dfs = func(index int) {
		visited[nums[index]] = true
		for _, son := range tree[index] {
			if visited[nums[son]] {
				continue
			}
			dfs(son)
		}
	}

	mex := 2
	for node != -1 {
		dfs(node)
		for visited[mex] {
			mex++
		}
		ans[node] = mex
		node = parents[node]
	}

	return ans
}
