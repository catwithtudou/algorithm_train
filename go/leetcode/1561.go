package leetcode

import "golang.org/x/exp/slices"

func maxCoins1561(piles []int) (ans int) {
	slices.Sort(piles)
	n := len(piles)
	for i := n / 3; i < n; i += 2 {
		ans += piles[i]
	}

	return
}
