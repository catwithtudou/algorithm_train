package leetcode

func minFlips1888(s string) int {
	n := len(s)
	ans := n
	cnt := 0
	for i := range 2*n - 1 {
		if int(s[i%n]%2) != i%2 {
			cnt++
		}
		left := i - n + 1
		if left < 0 {
			continue
		}
		ans = min(ans, min(cnt, n-cnt))
		if int(s[left]%2) != left%2 {
			cnt--
		}
	}
	return ans
}
