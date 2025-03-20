package leetcode

import "github.com/emirpasic/gods/v2/trees/redblacktree"

func minReverseOperations(n int, p int, banned []int, k int) []int {
	ban := map[int]struct{}{p: {}}

	for _, x := range banned {
		ban[x] = struct{}{}
	}

	indices := [2]*redblacktree.Tree[int, struct{}]{
		redblacktree.New[int, struct{}](),
		redblacktree.New[int, struct{}](),
	}

	for i := range n {
		if _, ok := ban[i]; !ok {
			indices[i%2].Put(i, struct{}{})
		}
	}
	indices[0].Put(n, struct{}{})
	indices[1].Put(n, struct{}{})

	ans := make([]int, n)
	for i := range ans {
		ans[i] = -1
	}
	ans[p] = 0
	q := []int{p}

	for len(q) > 0 {
		i := q[0]
		q = q[1:]

		mn := max(i-k+1, k-i-1)
		mx := min(i+k-1, 2*n-k-i-1)
		t := indices[mn%2]

		for node, _ := t.Ceiling(mn); node.Key <= mx; node, _ = t.Ceiling(mn) {
			j := node.Key
			ans[j] = ans[i] + 1
			q = append(q, j)
			t.Remove(j)
		}
	}

	return ans
}
