package leetcode

func numberOfAlternatingGroupsII(colors []int, k int) int {
	ans, cnt := 0, 0
	n := len(colors)
	for i := 0; i < 2*n; i++ {
		if i > 0 && colors[i%n] == colors[(i-1)%n] {
			cnt = 0
		}
		cnt++
		if i >= n && cnt >= k {
			ans++
		}
	}
	return ans
}
