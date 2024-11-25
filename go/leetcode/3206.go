package leetcode

func numberOfAlternatingGroups(colors []int) int {
	n := len(colors)
	cnt, ans := 0, 0
	for i := 0; i < n*2; i++ {
		if i > 0 && colors[i%n] == colors[(i-1)%n] {
			cnt = 0
		}
		cnt++
		if i >= n && cnt >= 3 {
			ans++
		}
	}
	return ans
}
