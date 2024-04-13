package leetcode

func findChampionLevel2(n int, edges [][]int) int {

	loser := make([]bool, n)
	for _, e := range edges {
		loser[e[1]] = true
	}

	ans := -1
	for i, w := range loser {
		if w {
			continue
		}
		if ans != -1 {
			return -1
		}
		ans = i
	}

	return ans
}
