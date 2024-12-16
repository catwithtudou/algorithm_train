package leetcode

import (
	"math"

	"github.com/emirpasic/gods/v2/trees/redblacktree"
	"golang.org/x/exp/slices"
)

func closestRoom(rooms [][]int, queries [][]int) []int {
	slices.SortFunc(rooms, func(a, b []int) int { return b[1] - a[1] })

	q := len(queries)
	queryIds := make([]int, q)
	for i := range queryIds {
		queryIds[i] = i
	}
	slices.SortFunc(queryIds, func(i, j int) int { return queries[j][1] - queries[i][1] })

	ans := make([]int, q)
	for i := range ans {
		ans[i] = -1
	}

	roomIds := redblacktree.New[int, struct{}]()
	j := 0
	for _, i := range queryIds {
		preferId, minSize := queries[i][0], queries[i][1]
		for j < len(rooms) && rooms[j][1] >= minSize {
			roomIds.Put(rooms[j][0], struct{}{})
			j++
		}

		diff := math.MaxInt
		if node, ok := roomIds.Floor(preferId); ok {
			diff = preferId - node.Key
			ans[i] = node.Key
		}
		if node, ok := roomIds.Ceiling(preferId); ok && node.Key-preferId < diff {
			ans[i] = node.Key
		}
	}
	return ans
}
