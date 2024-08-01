package lcp

import "golang.org/x/exp/slices"

func maxmiumScore(cards []int, cnt int) int {
	slices.SortFunc(cards, func(a, b int) int {
		return b - a
	})

	s := 0
	for _, v := range cards[:cnt] {
		s += v
	}
	if s%2 == 0 {
		return s
	}

	replaceNumFunc := func(x int) int {
		for _, v := range cards[cnt:] {
			if v%2 != x%2 {
				return s - x + v
			}
		}
		return 0
	}

	x := cards[cnt-1]
	ans := replaceNumFunc(x)
	for i := cnt - 2; i >= 0; i-- {
		if cards[i]%2 != x%2 {
			ans = max(ans, replaceNumFunc(cards[i]))
			break
		}
	}

	return ans
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
