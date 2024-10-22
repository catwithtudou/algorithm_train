package leetcode

func countCompleteDayPairsII(hours []int) int64 {
	var ans int64
	const H = 24
	cnt := make([]int64, H)
	for _, v := range hours {
		ans += cnt[(H-v%H)%H]
		cnt[v%H]++
	}
	return ans
}
