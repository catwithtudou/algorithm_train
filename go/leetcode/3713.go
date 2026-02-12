package leetcode

func longestBalanced3713(s string) int {
	ans := 0
	n := len(s)
	for i := 0; i < n; i++ {
		cnt := make([]int, 26)
		mx, kinds := 0, 0
		for j := i; j < n; j++ {
			b := s[j] - 'a'
			if cnt[b] == 0 {
				kinds++
			}
			cnt[b]++
			mx = max(mx, cnt[b])
			if mx*kinds == j-i+1 {
				ans = max(ans, j-i+1)
			}
		}
	}

	return ans
}
