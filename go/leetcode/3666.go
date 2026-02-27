package leetcode

import (
	"strings"

	"github.com/emirpasic/gods/v2/trees/redblacktree"
)

func minOperations3666(s string, k int) (ans int) {
	n := len(s)
	notVis := [2]*redblacktree.Tree[int, struct{}]{}
	for m := range notVis {
		notVis[m] = redblacktree.New[int, struct{}]()
		for i := m; i <= n; i += 2 {
			notVis[m].Put(i, struct{}{})
		}
		notVis[m].Put(n+1, struct{}{})
	}

	start := strings.Count(s, "0")
	notVis[start%2].Remove(start)
	q := []int{start}
	for q != nil {
		tmp := q
		q = nil
		for _, z := range tmp {
			if z == 0 {
				return ans
			}

			mn := z + k - 2*min(k, z)
			mx := z + k - 2*max(0, k-n+z)
			t := notVis[mn%2]
			for node, _ := t.Ceiling(mn); node.Key <= mx; node, _ = t.Ceiling(mn) {
				q = append(q, node.Key)
				t.Remove(node.Key)
			}
		}
		ans++
	}

	return -1
}
