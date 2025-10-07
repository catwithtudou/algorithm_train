package leetcode

import "github.com/emirpasic/gods/v2/trees/redblacktree"

func avoidFlood(rains []int) []int {
	ans := make([]int, len(rains))
	fullDay := make(map[int]int)
	dryDay := redblacktree.New[int, struct{}]()
	for i, lake := range rains {
		if lake == 0 {
			ans[i] = 1
			dryDay.Put(i, struct{}{})
			continue
		}
		if j, ok := fullDay[lake]; ok {
			node, _ := dryDay.Ceiling(j)
			if node == nil {
				return nil
			}
			ans[node.Key] = lake
			dryDay.Remove(node.Key)
		}
		ans[i] = -1
		fullDay[lake] = i
	}
	return ans
}
