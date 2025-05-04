package leetcode

func numEquivDominoPairs(dominoes [][]int) (ans int) {

	cnt := [10][10]int{}

	for _, d := range dominoes {
		a, b := min(d[0], d[1]), max(d[0], d[1])
		ans += cnt[a][b]
		cnt[a][b]++
	}

	return
}
