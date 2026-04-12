package leetcode

import "math"

var dis [26][26]int

func minimumDistance1320Init() {
	const column = 6
	for i := range 26 {
		for j := range 26 {
			dis[i][j] = abs(i/column-j/column) + abs(i%column-j%column)
		}
	}
}

func minimumDistance1320(word string) int {
	n := len(word)
	memo := make([][26][26]int, n)

	var dfs func(int, byte, byte) int

	dfs = func(i int, finger1, finger2 byte) (res int) {
		if i < 0 {
			return 0
		}

		p := &memo[i][finger1][finger2]
		if *p != 0 {
			return *p - 1
		}
		defer func() {
			*p = res + 1
		}()

		w := word[i] - 'A'
		res1 := dfs(i-1, w, finger2) + dis[finger1][w]

		res2 := dfs(i-1, finger1, w) + dis[finger2][w]

		return min(res1, res2)
	}

	ans := math.MaxInt

	for finger2 := range byte(26) {
		ans = min(ans, dfs(n-2, word[n-1]-'A', finger2))
	}
	return ans
}
