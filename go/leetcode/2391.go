package leetcode

func garbageCollection(garbage []string, travel []int) int {
	ans := len(garbage[0])
	seen := map[rune]struct{}{}
	for i := len(garbage) - 1; i > 0; i-- {
		g := garbage[i]
		for _, v := range g {
			seen[v] = struct{}{}
		}
		ans += len(g) + travel[i-1]*len(seen)
	}

	return ans
}
