package leetcode

func countCompleteDayPairs(hours []int) int {
	ans := 0
	const H = 24
	cnt := make([]int, H)
	for _, v := range hours {
		ans += cnt[(H-v%H)%H]
		cnt[v%H]++
	}
	return ans
}
