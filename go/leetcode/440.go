package leetcode

func findKthNumber(n int, k int) int {
	countSubtreeSize := func(node int) (size int) {
		first, last := node, node+1
		for first <= n {
			size += min(last, n+1) - first
			first *= 10
			last *= 10
		}
		return
	}

	node := 1
	k--
	for k > 0 {
		size := countSubtreeSize(node)
		if size <= k {
			node++
			k -= size
		} else {
			node *= 10
			k--
		}
	}
	return node
}
