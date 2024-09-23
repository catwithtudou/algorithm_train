package leetcode

func maxScoreSightseeingPair(values []int) int {
	mx := 0
	ans := 0
	for j, v := range values {
		ans = max(ans, mx+v-j)
		mx = max(mx, j+v)
	}
	return ans
}
